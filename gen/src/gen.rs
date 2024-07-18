use anyhow::Result;
use heck::ToTitleCase;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use skia_safe::{Color, Font, FontMgr, Paint, Rect};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::PathBuf,
    rc::Rc,
};

pub const FAMILY: &str = "Family";
pub const FONT: &str = "Font";
pub const CATEGORY: &str = "Category";
pub const TAKE: usize = 11; //usize::MAX;
pub const FAMILY_ID_INCREMENT: u32 = 1000; // The Roboto Serif font family has 721 fonts.

#[derive(Debug, Clone)]
pub struct Fam {
    pub id: u32,
    pub name: String,
    pub variant: String,
    pub cat: Rc<RefCell<Cat>>,
    pub meta: FamilyMetadata,
    pub fnts: Vec<Rc<RefCell<Fnt>>>,
}
#[derive(Debug, Clone)]
pub struct Fnt {
    pub id: u32,
    pub name: String,
    pub variant: String,
    pub fam: Rc<RefCell<Fam>>,
}

#[derive(Debug, Clone)]
pub struct Cat {
    pub name: String,
    pub variant: String,
    pub fams: Vec<Rc<RefCell<Fam>>>,
    pub fnts: Vec<Rc<RefCell<Fnt>>>,
}

pub fn rc<T>(v: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(v))
}

pub fn build(file_path: &str, is_in_prj_gen: bool) -> Result<()> {
    let cli = Client::new();

    // Get family metadata list from network.
    let fam_metas: Vec<FamilyMetadata> = get_family_metadata_list(&cli)?
        .into_iter()
        .take(TAKE)
        .collect();

    // Create category list.
    let mut cat_names: Vec<String> = fam_metas.iter().map(|o| o.category.clone()).collect();
    cat_names.sort_unstable();
    cat_names.dedup();
    let cats: Vec<Rc<RefCell<Cat>>> = cat_names
        .into_iter()
        .map(|o| {
            rc(Cat {
                name: o.clone(),
                variant: o.replace(' ', ""),
                fams: Vec::new(),
                fnts: Vec::new(),
            })
        })
        .collect();

    // Create families list.
    let mut fams: Vec<Rc<RefCell<Fam>>> = Vec::with_capacity(fam_metas.len());
    let mut id: u32 = 0;
    for fam_meta in fam_metas.iter() {
        let name = fam_meta.family.clone();
        let variant = name.replace(' ', "");
        let cat = cats
            .iter()
            .find(|o| o.borrow().name == fam_meta.category)
            .unwrap();
        let fam = rc(Fam {
            id,
            name,
            variant,
            cat: cat.clone(),
            meta: fam_meta.clone(),
            fnts: Vec::new(),
        });
        cat.borrow_mut().fams.push(fam.clone());
        fams.push(fam);
        id += FAMILY_ID_INCREMENT;
    }

    // Create font list.
    let mut fnts: Vec<Rc<RefCell<Fnt>>> = Vec::with_capacity(fam_metas.len());
    let mut fnt_variants_cnt: HashMap<String, u8> = HashMap::with_capacity(fam_metas.len());
    for fam in fams.iter_mut() {
        // Get file list for network.
        let fnt_fles = get_family_file_list(&fam.borrow().name, &cli)?;
        for (idx_fnt_fle, fnt_fle) in fnt_fles.iter().enumerate() {
            // Clean font name.
            // Remove file suffix `.ttf` from filename: ABeeZee-Regular.ttf.
            // eprintln!("{}", fnt_fle.filename);
            let mut name = fnt_fle.filename[..fnt_fle.filename.len() - 4]
                .replace("static/", "")
                .replace('_', "");
            if let Some(idx_fnd) = name.find("VariableFont") {
                name = name[..idx_fnd + 8].to_string();
            }

            // Create a font.
            let mut fnt = Fnt {
                id: idx_fnt_fle as u32,
                name: name.replace('-', " "),
                variant: name.replace('-', ""),
                fam: fam.clone(),
            };
            // Check for font name collision.
            // Occurs for "Asap CondensedLight", "AsapCondensed Light".
            *fnt_variants_cnt.entry(fnt.variant.clone()).or_default() += 1;
            if let Some(cnt) = fnt_variants_cnt.get(&fnt.variant) {
                if *cnt != 1 {
                    // Append numeric suffix to font name.
                    fnt.variant.push_str(&cnt.to_string());
                }
            }
            let fnt = rc(fnt);
            fam.borrow_mut().fnts.push(fnt.clone());
            fam.borrow_mut().cat.borrow_mut().fnts.push(fnt.clone());
            fnts.push(fnt);
        }
    }

    // Write files.

    // Write Error file.
    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_error(&mut buf);
    fs::write(format!("{}error.rs", file_path), buf)?;

    // Write Family file.
    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_family(&fams, &mut buf);
    fs::write(format!("{}family.rs", file_path), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_font(&fnts, &mut buf);
    fs::write(format!("{}font.rs", file_path), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_category(&cats, &mut buf);
    fs::write(format!("{}category.rs", file_path), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_lib(&fnts, &mut buf, is_in_prj_gen);
    let suffix = if is_in_prj_gen { "2" } else { "" };
    fs::write(format!("{}lib{}.rs", file_path, suffix), buf)?;

    wrt_fle_svgs(&fnts, &cli)?;

    // TODO: Designers (Add to font family doc comment, font doc comment)
    // TODO: dateAdded, lastModified (Add to font family doc comment, font doc comment)
    // TODO: Subsets (See metadata) Add is_subset()?
    // TODO: Add enum for each Subset.
    // TODO: Add function for each subset returning Vec<_>.
    // TODO: SVG generation.
    // TODO: SVG reference in doc comment.

    Ok(())
}

pub fn wrt_fle_family(fams: &[Rc<RefCell<Fam>>], buf: &mut String) {
    // Write enum.
    // pub enum Family {
    //     ABeeZee,
    // }
    buf.push_str(r#"
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::font::Font;
use crate::category::Category;

/// The _family id_ increment.
/// 
/// The Roboto Serif font family has 721 fonts.
pub const ID_INCREMENT: isize = 1000;

/// An _enumeration_ of [Google font](https://fonts.google.com) families.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
"#);
    buf.push_str(&format!("pub enum {} {{\n", FAMILY));
    for fam in fams.iter() {
        buf.push_str(&format!(
            "    /// The [{}](https://fonts.google.com/specimen/{}) font family.\n",
            fam.borrow().name,
            fam.borrow().name.replace(' ', "+"),
        ));
        buf.push_str(&format!(
            "    {} = {},\n",
            fam.borrow().variant,
            fam.borrow().id
        ));
    }
    buf.push_str("}\n");

    // Write impl Family.
    buf.push('\n');
    buf.push_str(&format!("impl {} {{\n", FAMILY));
    buf.push_str(
        r#"
    /// Returns the _id_ for the [`Family`].
    pub fn id(&self) -> isize {
        *self as isize
    }

    /// Returns the default [`Font`].
    pub fn font(&self) -> Font {
       Font::from_id(self.id()).unwrap()
    }

"#,
    );

    // Write `from_id`.
    // pub fn from_id(id: isize) -> Option<Self> {
    //     match id {
    //         0 => Some(Family::ABeeZee),
    //         1000 => Some(Family::ADLaMDisplay),
    //         _ => None,
    //     }
    // }
    buf.push_str("    /// Converts an `isize` to a [`Family`].\n");
    buf.push_str("    pub fn from_id(id: isize) -> Option<Self> {\n");
    buf.push_str("        match id {\n");
    for fam in fams.iter() {
        buf.push_str(&format!(
            "            {} => Some({}::{}),\n",
            fam.borrow().id,
            FAMILY,
            fam.borrow().variant
        ));
    }
    buf.push_str("            _ => None,\n");
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end from_id

    // Write `name`.
    // pub fn name(&self) -> String {
    //     self.as_ref().into()
    // }
    buf.push('\n');
    buf.push_str("    /// The name of the font [`Family`] with spaces.\n");
    buf.push_str("    pub fn name(&self) -> String {\n");
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        buf.push_str(&format!(
            "            {}::{} => \"{}\".into(),\n",
            FAMILY,
            fam.borrow().variant,
            fam.borrow().name
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n");

    // Write `fonts`.
    buf.push('\n');
    buf.push_str("    /// Returns fonts within the [`Family`].\n");
    buf.push_str(&format!("    pub fn fonts(&self) -> Vec<{}> {{\n", FONT));
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            FAMILY,
            fam.borrow().variant
        ));
        buf.push_str("                vec![\n");
        for fnt in fam.borrow().fnts.iter() {
            buf.push_str(&format!(
                "                    {}::{},\n",
                FONT,
                fnt.borrow().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end fonts

    // Write `category`.
    buf.push_str(&format!("    /// Returns the font [`{}`].\n", CATEGORY));
    buf.push_str(&format!(
        "    pub fn {}(&self) -> {} {{\n",
        CATEGORY.to_lowercase(),
        CATEGORY
    ));
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        buf.push_str(&format!(
            "            {}::{} => {}::{},\n",
            FAMILY,
            fam.borrow().variant,
            CATEGORY,
            fam.borrow().cat.borrow().variant
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `category`

    buf.push_str("}\n"); // end impl Family
}

pub fn wrt_fle_font(fnts: &[Rc<RefCell<Fnt>>], buf: &mut String) {
    // Write enum.
    // pub enum Font {
    //     ABeeZeeRegular,
    // }
    buf.push_str(r#"
use crate::category::Category;
use crate::error::{FontError, StringError};
use crate::family::{Family, ID_INCREMENT};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString};

/// An _enumeration_ of [Google fonts](https://fonts.google.com).
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
pub enum Font {
    "#);
    let mut id: u16 = 0;
    let mut fam_prv_o: Option<Rc<RefCell<Fam>>> = None;
    for fnt in fnts.iter() {
        if let Some(fam_prv) = fam_prv_o {
            if fnt.borrow().fam.borrow().variant == fam_prv.borrow().variant {
                id += 1;
            } else {
                id = 0;
            }
        }

        buf.push_str(&format!(
            "    /// The [{}](https://fonts.google.com/specimen/{}) _{}_ font.\n",
            fnt.borrow().fam.borrow().name,
            &fnt.borrow().fam.borrow().name.replace(' ', "+"),
            fnt.borrow().name_suffix()
        ));
        buf.push_str("    ///\n");
        buf.push_str(&format!(
            "    /// ![{}](../doc/{}.svg)\n",
            fnt.borrow().name,
            fnt.borrow().variant
        ));
        if id == 0 {
            buf.push_str(&format!(
                "    {} = Family::{} as isize,\n",
                fnt.borrow().variant,
                fnt.borrow().fam.borrow().variant
            ));
        } else {
            buf.push_str(&format!(
                "    {} = {} + Family::{} as isize,\n",
                fnt.borrow().variant,
                id,
                fnt.borrow().fam.borrow().variant
            ));
        }

        fam_prv_o = Some(fnt.borrow().fam.clone());
    }
    buf.push_str("}\n"); // end enum Font

    buf.push_str(
        r#"
impl Font {
    /// Returns the _id_ for the [`Font`].
    pub fn id(&self) -> isize {
        *self as isize
    }

    /// Returns the [`Family`].
    pub fn family(&self) -> Family {
        Family::from_id((self.id() / ID_INCREMENT) * ID_INCREMENT).unwrap()
    }

    /// Returns the index of the font file within the [`Family`].
    pub fn font_file_idx(&self) -> usize {
         (self.id() - self.family().id()) as usize 
    }

    /// Returns the name of the [`Font`].
    pub fn name(&self) -> String {
        self.as_ref().into()
    }    

    /// Indicates whether the [`Font`] uses _variable_ font technology.
    pub fn is_variable(&self) -> bool {
        self.as_ref().contains("Variable")
    }

    /// Indicates whether the [`Font`] uses _static_ font technology.
    pub fn is_static(&self) -> bool {
        !self.is_variable()
    }
"#,
    );

    // Write `get`.
    buf.push_str(
        r#"
    /// Get font data from the network.
    pub fn get(&self) -> Result<Vec<u8>, FontError> {
        // Get file info from the network.
        let result = Client::new()
            .get("https://fonts.google.com/download/list")
            .query(&[("family", self.family().name())])
            .send();
        match result {
            Err(e) => Err(FontError::Network(e)),
            Ok(response) => {
                match response.text() {
                    Err(e) => Err(FontError::Network(e)),
                    Ok(txt) => {
                        // Trim leading excess characters
                        // to allow deserialization.
                        //  ")]}'\n{\n
                        let mut txt: &str = txt.as_ref();
                        if let Some(idx) = txt.find('{') {
                            if idx != 0 {
                                txt = &txt[idx..];
                            }
                        }

                        // Deserialize the file info.
                        match serde_json::from_str::<FamilyFileList>(txt) {
                            Err(e) => Err(FontError::Deserialize(e)),
                            Ok(file_info) => {
                                // Get the file url.
                                let url = &file_info.manifest.file_refs[self.font_file_idx()].url;

                                // Get font file from the network.
                                let result = Client::new().get(url).send();
                                match result {
                                    Err(e) => Err(FontError::Network(e)),
                                    Ok(response) => match response.bytes() {
                                        Err(e) => Err(FontError::Network(e)),
                                        Ok(bytes) => Ok(bytes.to_vec()),
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get font data and store locally.
    pub fn get_and_cache(&self) -> Result<Vec<u8>, FontError> {
        // Get the cache directory.
        match dirs::cache_dir() {
            None => Err(FontError::CacheDir(StringError::new(
                "Missing cache directory",
            ))),
            Some(mut pth) => {
                // Set the file name.
                pth.push("google-fonts");
                pth.push(self.to_string()); // Font file name.
                pth.set_extension("ttf");

                if pth.exists() {
                    // Load the font file from disk.
                    match fs::read(pth) {
                        Err(e) => Err(FontError::CacheFile(e)),
                        Ok(font_data) => Ok(font_data),
                    }
                } else {
                    // Create the cache directory if necessary.
                    // Get the directory part of the path.
                    if let Some(directory) = pth.parent() {
                        // Check if the directory exists.
                        if !directory.exists() {
                            // Create the directory and any missing parent directories.
                            if let Err(e) = fs::create_dir_all(directory) {
                                return Err(FontError::CacheFile(e));
                            }
                        }
                    }

                    // Get the font data from the network.
                    match self.get() {
                        Err(err) => Err(err),
                        // Write the font data to disk.
                        Ok(font_data) => match fs::write(pth, &font_data) {
                            Err(e) => Err(FontError::CacheFile(e)),
                            Ok(_) => Ok(font_data),
                        },
                    }
                }
            }
        }
    }

"#,
    );

    // Write `from_id`.
    // pub fn from_id(id: isize) -> Option<Self> {
    //     match id {
    //         0 => Some(Font::ABeeZeeRegular),
    //         1 => Some(Font::ABeeZeeItalic),
    //         _ => None,
    //     }
    // }
    buf.push_str("    /// Converts an `isize` to a [`Font`].\n");
    buf.push_str("    pub fn from_id(id: isize) -> Option<Self> {\n");
    buf.push_str("        match id {\n");
    for fnt in fnts.iter() {
        buf.push_str(&format!(
            "            {} => Some({}::{}),\n",
            fnt.borrow().id + fnt.borrow().fam.borrow().id,
            FONT,
            fnt.borrow().variant
        ));
    }
    buf.push_str("            _ => None,\n");
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end from_id

    // Write `category`.
    buf.push_str(&format!("    /// Returns the font [`{}`].\n", CATEGORY));
    buf.push_str(&format!(
        "    pub fn {}(&self) -> {} {{\n",
        CATEGORY.to_lowercase(),
        CATEGORY
    ));
    buf.push_str("        match self {\n");
    for fnt in fnts.iter() {
        buf.push_str(&format!(
            "            {}::{} => {}::{},\n",
            FONT,
            fnt.borrow().variant,
            CATEGORY,
            fnt.borrow().fam.borrow().cat.borrow().variant
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `category`

    buf.push_str("}\n"); // end impl Font

    // Write supporting structs.
    buf.push_str(
        r#"
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyFileList {
    manifest: Manifest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    file_refs: Vec<FileRef>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    url: String,
}
    "#,
    );
}

pub fn wrt_fle_category(cats: &[Rc<RefCell<Cat>>], buf: &mut String) {
    // Write enum.
    // pub enum Category {
    //     ABeeZee,
    // }
    buf.push_str(r#"
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::family::Family;
use crate::font::Font;

/// An _enumeration_ of font categories.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
"#);
    buf.push_str(&format!("pub enum {} {{\n", CATEGORY));
    for cat in cats.iter() {
        buf.push_str(&format!(
            "    /// The _{}_ font category.\n",
            cat.borrow().name
        ));
        buf.push_str(&format!("    {},\n", cat.borrow().variant));
    }
    buf.push_str("}\n");

    // Write impl Category.
    buf.push('\n');
    buf.push_str(&format!("impl {} {{\n", CATEGORY));

    // Write `name`.
    // pub fn name(&self) -> String {
    //     self.as_ref().into()
    // }
    buf.push('\n');
    buf.push_str("    /// The name of the font [`Category`] with spaces.\n");
    buf.push_str("    pub fn name(&self) -> String {\n");
    buf.push_str("        match self {\n");
    for fam in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => \"{}\".into(),\n",
            CATEGORY,
            fam.borrow().variant,
            fam.borrow().name
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n");

    // Write `families`.
    buf.push('\n');
    buf.push_str("    /// Returns families within the [`Category`].\n");
    buf.push_str(&format!(
        "    pub fn families(&self) -> Vec<{}> {{\n",
        FAMILY
    ));
    buf.push_str("        match self {\n");
    for cat in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            CATEGORY,
            cat.borrow().variant
        ));
        buf.push_str("                vec![\n");
        for fam in cat.borrow().fams.iter() {
            buf.push_str(&format!(
                "                    {}::{},\n",
                FAMILY,
                fam.borrow().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `families`

    // Write `fonts`.
    buf.push('\n');
    buf.push_str("    /// Returns fonts within the [`Category`].\n");
    buf.push_str(&format!("    pub fn fonts(&self) -> Vec<{}> {{\n", FONT));
    buf.push_str("        match self {\n");
    for cat in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            CATEGORY,
            cat.borrow().variant
        ));
        buf.push_str("                vec![\n");
        for fnt in cat.borrow().fnts.iter() {
            buf.push_str(&format!(
                "                    {}::{},\n",
                FONT,
                fnt.borrow().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `fonts`

    buf.push_str("}\n"); // end impl Family
}

pub fn wrt_fle_error(buf: &mut String) {
    buf.push_str(
        r#"
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// An error that can occur while using the `google-fonts` crate.
///
/// This enum represents various errors that can be encountered while
/// downloading, deserializing, caching, or otherwise handling fonts
/// from Google Fonts. Each variant corresponds to a specific type
/// of error that can occur during the process.
///
/// # Variants
///
/// - `Network`: Indicates an error that occurred while making a network request.
/// - `Deserialize`: Indicates an error that occurred while deserializing JSON data.
/// - `CacheDir`: Indicates an error that occurred while interacting with the cache directory.
/// - `CacheFile`: Indicates an error that occurred while interacting with a cache file.
#[derive(Debug)]
pub enum FontError {
    /// An error that occurred while making a network request.
    ///
    /// This variant wraps a `reqwest::Error`, which provides more details
    /// about the specific network-related error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::Network(e) = error {
    ///     println!("Network error: {}", e);
    /// }
    /// ```
    Network(reqwest::Error),

    /// An error that occurred while deserializing JSON data.
    ///
    /// This variant wraps a `serde_json::Error`, which provides more details
    /// about the specific deserialization error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::Deserialize(e) = error {
    ///     println!("Deserialization error: {}", e);
    /// }
    /// ```
    Deserialize(serde_json::Error),

    /// An error that occurred while interacting with the cache directory.
    ///
    /// This variant wraps a `StringError`, which provides more details
    /// about the specific error related to the cache directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::CacheDir(e) = error {
    ///     println!("Cache directory error: {}", e);
    /// }
    /// ```
    CacheDir(StringError),

    /// An error that occurred while interacting with a cache file.
    ///
    /// This variant wraps a `std::io::Error`, which provides more details
    /// about the specific I/O error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::CacheFile(e) = error {
    ///     println!("Cache file error: {}", e);
    /// }
    /// ```
    CacheFile(std::io::Error),
}

impl Display for FontError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FontError::Network(e) => write!(f, "font network error: {}", e),
            FontError::Deserialize(e) => write!(f, "deserialization error: {}", e),
            FontError::CacheDir(e) => write!(f, "font cache directory error: {}", e),
            FontError::CacheFile(e) => write!(f, "font cache file error: {}", e),
        }
    }
}

impl Error for FontError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FontError::Network(e) => Some(e),
            FontError::Deserialize(e) => Some(e),
            FontError::CacheDir(e) => Some(e),
            FontError::CacheFile(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct StringError {
    msg: String,
}
impl StringError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
impl Display for StringError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}
impl Error for StringError {}

"#,
    );
}

pub fn wrt_fle_lib(fnts: &[Rc<RefCell<Fnt>>], buf: &mut String, is_in_prj_gen: bool) {
    if is_in_prj_gen {
        buf.push_str(
            r#"
// pub mod category;
// pub mod error;
// pub mod family;
// pub mod font;
    "#,
        );
    } else {
        buf.push_str(
            r#"
pub mod category;
pub mod error;
pub mod family;
pub mod font;
    "#,
        );
    }
    buf.push_str(
        r#"
use crate::font::Font;
use crate::error::FontError;

"#,
    );

    // Write individual font functions.
    for fnt in fnts.iter() {
        buf.push('\n');
        buf.push_str(&format!(
            "/// Get font data for the [{}](https://fonts.google.com/specimen/{}) _{}_ font.\n",
            fnt.borrow().fam.borrow().name,
            fnt.borrow().fam.borrow().name.replace(' ', "+"),
            fnt.borrow().name_suffix(),
        ));
        buf.push_str("///\n");
        buf.push_str("/// Loaded from the network and cached to disk.\n");
        buf.push_str(&format!(
            "pub fn {}() -> Result<Vec<u8>, FontError> {{\n",
            fnt.borrow().fn_name()
        ));
        buf.push_str(&format!(
            "    {}::{}.get_and_cache()\n",
            FONT,
            fnt.borrow().variant
        ));
        buf.push_str("}\n");
    }

    // Write `variable_fonts`
    buf.push('\n');
    buf.push_str("/// Fonts which use _variable_ font technology.\n");
    buf.push_str(&format!("pub fn variable_fonts() -> Vec<{}> {{\n", FONT));
    buf.push_str("    vec![\n");
    for fnt in fnts
        .iter()
        .filter(|o| o.borrow().variant.contains("Variable"))
    {
        buf.push_str(&format!("        {}::{},\n", FONT, fnt.borrow().variant));
    }
    buf.push_str("    ]\n");
    buf.push_str("}\n");

    // Write `static_fonts`.
    buf.push('\n');
    buf.push_str("/// Fonts which use _static_ font technology.\n");
    buf.push_str(&format!("pub fn static_fonts() -> Vec<{}> {{\n", FONT));
    buf.push_str("    vec![\n");
    for fnt in fnts
        .iter()
        .filter(|o| !o.borrow().variant.contains("Variable"))
    {
        buf.push_str(&format!("        {}::{},\n", FONT, fnt.borrow().variant));
    }
    buf.push_str("    ]\n");
    buf.push_str("}\n");

    // Write `tests` module.
    buf.push('\n');
    buf.push_str("#[cfg(test)]\n");
    buf.push_str("mod tests {\n");
    buf.push_str("    use super::*;\n");
    buf.push_str("    use ttf_parser::Face;\n");
    for fnt in fnts.iter() {
        buf.push('\n');
        buf.push_str("    #[test]\n");
        buf.push_str(&format!("    fn test_{}() {{\n", fnt.borrow().fn_name()));
        buf.push_str(&format!(
            "        let result = {}();\n",
            fnt.borrow().fn_name()
        ));
        buf.push_str("        assert!(result.is_ok());\n");
        buf.push_str("        let font_data = result.unwrap();\n");
        buf.push_str("        let result2 = Face::parse(&font_data, 0);\n");
        buf.push_str("        assert!(result2.is_ok());\n");
        buf.push_str("    }\n");
    }
    buf.push_str("}\n"); // end mod tests
}

pub fn wrt_fle_svgs(fnts: &[Rc<RefCell<Fnt>>], cli: &Client) -> Result<()> {
    let mgr = FontMgr::new();
    let mut paint1 = Paint::default();
    paint1.set_color(Color::from_rgb(255, 255, 255));
    for fnt in fnts.iter() {
        let fnt_dat = fnt.borrow().get(cli)?;
        let face = mgr.new_from_data(&fnt_dat, None).unwrap();
        let font = &Font::from_typeface(face, 18.0);
        let name = &fnt.borrow().name;
        let (_, mut rect) = font.measure_str(name, Some(&paint1));
        let mrg: f32 = 3.0;
        rect.left -= mrg;
        rect.right += mrg;
        rect.bottom += mrg;
        rect.top -= mrg;
        let size = rect.size();
        // eprintln!("{:?} {:?}", rect, size);
        let canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
        canvas.draw_str(name, (mrg, size.height - rect.bottom), font, &paint1);
        let data = canvas.end();
        fs::write(format!("../doc/imgs/{}.svg", fnt.borrow().variant), data.as_bytes())?;
    }

    Ok(())
}

// Request URL: https://fonts.google.com/metadata/fonts/Roboto
// Request Method:GET

// Request URL: https://fonts.google.com/metadata/fonts/Playwrite%20CU
// Request Method: GET

// Request URL: https://fonts.google.com/metadata/fonts/Roboto%20Condensed
// Request Method: GET

// Request URL: https://fonts.google.com/sampletext?family=Roboto&paragraphOnly=true
// Request Method: GET

// https://www.gstatic.com/images/icons/material/apps/fonts/1x/catalog/checkout/google_web.png=w200

/// ![Google Fonts](https://www.gstatic.com/images/icons/material/apps/fonts/1x/catalog/checkout/google_web.png=w200)

pub const METADATA_LIST_URL: &str = "https://fonts.google.com/metadata/fonts";

/// Get a metadata list for font families.
pub fn get_family_metadata_list(cli: &Client) -> Result<Vec<FamilyMetadata>> {
    let txt = cli.get(METADATA_LIST_URL).send()?.text()?;
    // let mut pth = dirs::document_dir().unwrap();
    // pth.push("meta.json");
    // fs::write(pth, txt.as_bytes())?;

    // Deserialize JSON to struct.
    let ret: FamilyMetadataList = serde_json::from_str(&txt)?;

    Ok(ret.family_metadata_list)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMetadataList {
    pub family_metadata_list: Vec<FamilyMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMetadata {
    pub family: String,
    pub display_name: Option<String>,
    pub category: String,
    pub stroke: Option<String>,
    pub classifications: Vec<String>,
    pub size: i32,
    pub subsets: Vec<String>,
    pub fonts: Fonts,
    pub axes: Vec<Axis>,
    pub designers: Vec<String>,
    pub last_modified: String,
    pub date_added: String,
    pub popularity: i32,
    pub trending: i32,
    pub default_sort: i32,
    pub android_fragment: Option<String>,
    pub is_noto: bool,
    pub color_capabilities: Vec<String>,
    pub primary_script: Option<String>,
    pub primary_language: Option<String>,
    pub is_open_source: bool,
    pub is_brand_font: bool,
}
// Implement PartialEq
impl PartialEq for FamilyMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.family == other.family
    }
}

// Implement Eq
impl Eq for FamilyMetadata {}

// Implement PartialOrd
impl PartialOrd for FamilyMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.family.cmp(&other.family))
    }
}

// Implement Ord
impl Ord for FamilyMetadata {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.family.cmp(&other.family)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fallback {
    pub name: String,
    pub value: f32,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fonts {
    #[serde(rename = "100")]
    pub normal_100: Option<FontDetails>,
    #[serde(rename = "100i")]
    pub italic_100i: Option<FontDetails>,

    #[serde(rename = "200")]
    pub normal_200: Option<FontDetails>,
    #[serde(rename = "200i")]
    pub italic_200i: Option<FontDetails>,

    #[serde(rename = "300")]
    pub normal_300: Option<FontDetails>,
    #[serde(rename = "300i")]
    pub italic_300i: Option<FontDetails>,

    #[serde(rename = "400")]
    pub normal_400: Option<FontDetails>,
    #[serde(rename = "400i")]
    pub italic_400i: Option<FontDetails>,

    #[serde(rename = "500")]
    pub normal_500: Option<FontDetails>,
    #[serde(rename = "500i")]
    pub italic_500i: Option<FontDetails>,

    #[serde(rename = "600")]
    pub normal_600: Option<FontDetails>,
    #[serde(rename = "600i")]
    pub italic_600i: Option<FontDetails>,

    #[serde(rename = "700")]
    pub normal_700: Option<FontDetails>,
    #[serde(rename = "700i")]
    pub italic_700i: Option<FontDetails>,

    #[serde(rename = "800")]
    pub normal_800: Option<FontDetails>,
    #[serde(rename = "800i")]
    pub italic_800i: Option<FontDetails>,

    #[serde(rename = "900")]
    pub normal_900: Option<FontDetails>,
    #[serde(rename = "900i")]
    pub italic_900i: Option<FontDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontDetails {
    pub thickness: Option<i32>,
    pub slant: Option<i32>,
    pub width: Option<i32>,
    pub line_height: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    pub tag: String,
    pub min: f32,
    pub default_value: f32,
    pub max: f32,
}

pub const FILE_LIST_URL: &str = "https://fonts.google.com/download/list";

pub fn cache_dir() -> PathBuf {
    let mut pth = dirs::cache_dir().unwrap();
    pth.push("google-fonts");
    pth.push("gen");

    pth
}

/// Get a file list for font families.
pub fn get_family_file_list(family: &str, cli: &Client) -> Result<Vec<FileRef>> {
    // Create file path.
    let mut pth = cache_dir();
    pth.push(family.replace(' ', ""));
    pth.set_extension("json");

    // Check if the file already exists.
    if pth.exists() {
        // Load the file.
        let fle = File::open(pth)?;
        let rdr = BufReader::new(fle);

        // Deserialize the JSON into a struct.
        let ret: FamilyFileList = serde_json::from_reader(rdr)?;
        return Ok(ret.manifest.file_refs);
    }

    let txt = cli
        .get(FILE_LIST_URL)
        .query(&[("family", family)])
        .send()?
        .text()?;
    // let mut pth = dirs::document_dir().unwrap();
    // pth.push("meta.json");
    // fs::write(pth, txt.as_bytes())?;

    // Trim leading excess characters
    // to allow deserialization.
    //  ")]}'\n{\n
    let mut txt: &str = txt.as_ref();
    if let Some(idx) = txt.find('{') {
        if idx != 0 {
            txt = &txt[idx..];
        }
    }

    // Create the cache directory if necessary.
    // Get the directory part of the path.
    if let Some(directory) = pth.parent() {
        // Check if the directory exists.
        if !directory.exists() {
            // Create the directory and any missing parent directories.
            fs::create_dir_all(directory).unwrap();
        }
    }

    // Write the data to disk for caching.
    // eprintln!("writing {:?}", &pth);
    fs::write(pth, txt)?;

    // Deserialize JSON to struct.
    let ret: FamilyFileList = serde_json::from_str(txt)?;

    Ok(ret.manifest.file_refs)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyFileList {
    manifest: Manifest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    file_refs: Vec<FileRef>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    filename: String,
    url: String,
    date: Date,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    seconds: i64,
    nanos: i32,
}

impl Fnt {
    /// Get the font name suffix. For example, _regular_.
    pub fn name_suffix(&self) -> String {
        self.name
            .replace(&self.fam.borrow().variant, "")
            .trim()
            .to_title_case()
            .to_lowercase()
    }

    // Get the font function name.
    pub fn fn_name(&self) -> String {
        let suffix = self
            .variant
            .replace(&self.fam.borrow().variant, "")
            .trim()
            .to_title_case()
            .to_lowercase();
        format!("{} {}", self.fam.borrow().name.to_lowercase(), suffix).replace(' ', "_")
    }

    /// Get the font data from network or cache.
    pub fn get(&self, cli: &Client) -> Result<Vec<u8>> {
        // Create file path.
        let mut pth = cache_dir();
        pth.push(&self.variant);
        pth.set_extension("ttf");

        // Load cached file if exists.
        if pth.exists() {
            let mut file = File::open(pth)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            return Ok(buffer);
        }

        let file_refs = get_family_file_list(&self.fam.borrow().name, cli)?;

        // Get the font index.
        let (idx, _) = self
            .fam
            .borrow()
            .fnts
            .iter()
            .enumerate()
            .find(|(_, o)| o.borrow().name == self.name)
            .unwrap();

        // Get the font file url.
        let url = &file_refs[idx].url;

        // Get font file from the network.
        let byt = Client::new().get(url).send()?.bytes()?.to_vec();

        // Write bytes to file for caching.
        let mut file = File::create(pth)?;
        file.write_all(&byt)?;

        Ok(byt)
    }
}
