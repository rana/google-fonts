use anyhow::Result;
use heck::ToTitleCase;
use lazy_static::lazy_static;
use rayon::prelude::*;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use skia_safe::{surfaces, Color, EncodedImageFormat, Font, FontMgr, Paint};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub const FAMILY: &str = "Family";
pub const FONT: &str = "Font";
pub const CATEGORY: &str = "Category";
pub const SUBSET: &str = "Subset";
pub const VARIABLE: &str = "variable";
pub const STATIC: &str = "static";
pub const FULL: &str = "full";
pub const TAKE: usize = usize::MAX;
pub const FAMILY_ID_INCREMENT: u32 = 1000; // The Roboto Serif font family has 721 fonts.
pub const MAX_RETRIES: usize = 9;
pub const RETRY_DELAY: Duration = Duration::from_millis(500);

pub fn main() -> Result<()> {
    build("../fnt/src/", true)
}

pub fn build(pth: &str, wrt_imgs: bool) -> Result<()> {
    let cli = Client::new();

    // Get family metadata list from network.
    let fam_metas: Vec<FamilyMetadata> = get_family_metadata_list(&cli)?
        .into_iter()
        .take(TAKE)
        // .filter(|o| o.family.contains("Jomhuria"))
        .collect();

    // Create category list.
    let mut cat_names: Vec<String> = fam_metas.iter().map(|o| o.category.clone()).collect();
    cat_names.sort_unstable();
    cat_names.dedup();
    let cats: Vec<Arc<RwLock<Cat>>> = cat_names
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

    // Create subset list.
    let mut subset_names: Vec<String> = fam_metas.iter().flat_map(|o| o.subsets.clone()).collect();
    subset_names.sort_unstable();
    subset_names.dedup();
    let subs: Vec<Arc<RwLock<Sub>>> = subset_names
        .into_iter()
        .map(|o| {
            rc(Sub {
                name: o.clone(),
                variant: sub_variant(o),
                fams: Vec::new(),
                fnts: Vec::new(),
            })
        })
        .collect();

    // Create families list.
    let mut fams: Vec<Arc<RwLock<Fam>>> = Vec::with_capacity(fam_metas.len());
    let mut id: u32 = 0;
    for fam_meta in fam_metas.iter() {
        // Find Cat.
        let cat = cats
            .iter()
            .find(|o| o.read().unwrap().name == fam_meta.category)
            .unwrap();

        // Create Fam.
        let name = fam_meta.family.clone();
        let variant = name.replace(' ', "");
        let fam = rc(Fam {
            id,
            name,
            variant,
            cat: cat.clone(),
            meta: fam_meta.clone(),
            fnts: Vec::new(),
            subs: Vec::new(),
        });

        // Associate Cat.
        cat.write().unwrap().fams.push(fam.clone());

        // Associate Subs.
        for sub in subs
            .iter()
            .filter(|o| fam_meta.subsets.contains(&o.read().unwrap().name))
        {
            sub.write().unwrap().fams.push(fam.clone());
            fam.write().unwrap().subs.push(sub.clone());
        }

        fams.push(fam);
        id += FAMILY_ID_INCREMENT;
    }

    // Create font list.
    let mut fnts: Vec<Arc<RwLock<Fnt>>> = Vec::with_capacity(fam_metas.len());
    let mut fnt_variants_cnt: HashMap<String, u8> = HashMap::with_capacity(fam_metas.len());
    for fam in fams.iter_mut() {
        // Get file list for network.
        let fnt_fles = fam.read().unwrap().get_file_list(&cli)?.file_refs;
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
                subs: Vec::new(),
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

            // Associate Subs.
            for sub in subs.iter().filter(|o| {
                fnt.read()
                    .unwrap()
                    .fam
                    .read()
                    .unwrap()
                    .meta
                    .subsets
                    .contains(&o.read().unwrap().name)
            }) {
                sub.write().unwrap().fnts.push(fnt.clone());
                fnt.write().unwrap().subs.push(sub.clone());
            }

            fam.write().unwrap().fnts.push(fnt.clone());
            fam.write()
                .unwrap()
                .cat
                .write()
                .unwrap()
                .fnts
                .push(fnt.clone());
            fnts.push(fnt);
        }
    }

    // Write files.

    // Write Error file.
    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_error(&mut buf);
    fs::write(format!("{}error.rs", pth), buf)?;

    // Write Family file.
    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_family(&fams, &cli, &mut buf)?;
    fs::write(format!("{}family.rs", pth), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_font(&fnts, &mut buf);
    fs::write(format!("{}font.rs", pth), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_category(&cats, &mut buf);
    fs::write(format!("{}category.rs", pth), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_subset(&subs, &mut buf);
    fs::write(format!("{}subset.rs", pth), buf)?;

    let mut buf = String::with_capacity(1 << 20); // 1MB
    wrt_fle_lib(&fnts, &mut buf);
    fs::write(format!("{}lib.rs", pth), buf)?;

    if wrt_imgs {
        wrt_fle_imgs(&fnts, &cli)?;
    }

    wrt_fle_cargo_toml(pth)?;

    Ok(())
}

pub fn wrt_fle_family(fams: &[Arc<RwLock<Fam>>], cli: &Client, buf: &mut String) -> Result<()> {
    // Write enum.
    // pub enum Family {
    //     ABeeZee,
    // }
    buf.push_str(r#"
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::font::Font;
use crate::category::Category;
use crate::subset::Subset;

/// The _family id_ increment.
/// 
/// The Roboto Serif font family has 721 fonts.
pub const ID_INCREMENT: u32 = 1000;

/// An _enumeration_ of [Google font](https://fonts.google.com) families.
/// 
/// A font family may have one or more fonts with different styles and sizes.
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
#[repr(u32)] // for `mem::transmute`
"#);
    buf.push_str(&format!("pub enum {} {{\n", FAMILY));
    for fam in fams.iter() {
        // Get family description.
        let dtl = fam.read().unwrap().get_metadata_detail(cli)?;
        // Write title sentence.
        buf.push_str(&format!(
            "    /// The [{}](https://fonts.google.com/specimen/{}) font family.\n",
            fam.read().unwrap().name,
            fam.read().unwrap().name.replace(' ', "+"),
        ));
        // Write image.
        for fnt in fam.read().unwrap().fnts.iter().take(9) {
            buf.push_str("    ///\n");
            buf.push_str(&format!(
                "    /// ![{}](https://rana.github.io/google-fonts/doc/imgs/{}.webp)\n",
                fnt.read().unwrap().name,
                fnt.read().unwrap().variant
            ));
        }
        // Write description.
        let html = unescaper::unescape(&dtl.description).unwrap();
        let fragment = Html::parse_fragment(&html);
        let selector = Selector::parse("p").unwrap();
        for elm in fragment.select(&selector) {
            let txt = elm.inner_html().trim().replace('\n', " ");
            buf.push_str("    ///\n");
            buf.push_str(&format!("    /// {}\n", txt));
        }
        // Write designer.
        buf.push_str("    ///\n");
        buf.push_str(&format!(
            "    /// Designed by {}.\n",
            comma_and(&fam.read().unwrap().meta.designers)
        ));
        // Write variant.
        buf.push_str(&cfg_feature("    ", fam.read().unwrap().features()));
        buf.push_str(&format!(
            "    {} = {},\n",
            fam.read().unwrap().variant,
            fam.read().unwrap().id
        ));
    }
    buf.push_str("}\n");

    // Write impl Family.
    buf.push('\n');
    buf.push_str(&format!("impl {} {{\n", FAMILY));
    buf.push_str(
        r#"
    /// Returns the _id_ for the [`Family`].
    pub fn id(&self) -> u32 {
        *self as u32
    }

    /// Returns the first [`Font`].
    pub fn font(&self) -> Font {
       Font::from_id(self.id())
    }

"#,
    );

    // Write `from_id`.
    buf.push_str("    /// Transforms an _id_ into a [`Family`].\n");
    buf.push_str("    pub(crate) fn from_id(id: u32) -> Self {\n");
    buf.push_str("        unsafe { std::mem::transmute(id) }\n");
    buf.push_str("    }\n"); // end from_id

    // Write `name`.
    buf.push('\n');
    buf.push_str("    /// The name of the font [`Family`] with spaces.\n");
    buf.push_str("    pub fn name(&self) -> String {\n");
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        buf.push_str(&cfg_feature("            ", fam.read().unwrap().features()));
        buf.push_str(&format!(
            "            {}::{} => \"{}\".into(),\n",
            FAMILY,
            fam.read().unwrap().variant,
            fam.read().unwrap().name
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n");

    // Write `fonts`.
    buf.push('\n');
    buf.push_str("    /// Returns fonts for the [`Family`].\n");
    buf.push_str(&format!("    pub fn fonts(&self) -> Vec<{}> {{\n", FONT));
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        buf.push_str(&cfg_feature("            ", fam.read().unwrap().features()));
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            FAMILY,
            fam.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");
        for fnt in fam.read().unwrap().fnts.iter() {
            buf.push_str(&cfg_feature(
                "                    ",
                fnt.read().unwrap().features(),
            ));
            buf.push_str(&format!(
                "                    {}::{},\n",
                FONT,
                fnt.read().unwrap().variant
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
        buf.push_str(&cfg_feature("            ", fam.read().unwrap().features()));
        buf.push_str(&format!(
            "            {}::{} => {}::{},\n",
            FAMILY,
            fam.read().unwrap().variant,
            CATEGORY,
            fam.read().unwrap().cat.read().unwrap().variant
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `category`

    // Write `coverage`
    buf.push('\n');
    buf.push_str("    /// Unicode characters supported by the [`Family`].\n");
    buf.push_str(&format!(
        "    pub fn coverage(&self) -> Vec<({}, Vec<RangeInclusive<u32>>)> {{\n",
        SUBSET
    ));
    buf.push_str("        match self {\n");
    for fam in fams.iter() {
        let dtl = fam.read().unwrap().get_metadata_detail(cli)?;

        buf.push_str(&cfg_feature("            ", fam.read().unwrap().features()));
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            FAMILY,
            fam.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");

        let mut sorted_keys: Vec<_> = dtl.coverage.keys().cloned().collect::<Vec<_>>();
        sorted_keys.sort();
        for key in sorted_keys {
            buf.push_str(&format!(
                "                    ({}::{}, vec![",
                SUBSET,
                sub_variant(key.clone())
            ));

            // Parse unicode character ranges.
            for (idx, prt) in dtl.coverage.get(&key).unwrap().split(',').enumerate() {
                let mut prts = prt.split('-').map(|s| s.parse::<u32>().unwrap());
                let fst = prts.next().unwrap();
                let lst = match prts.next() {
                    Some(lst_val) => lst_val,
                    None => fst,
                };
                // ranges.push(start..=end);
                let cma = if idx == 0 { "" } else { "," };
                buf.push_str(&format!("{}{}..={}", cma, fst, lst));
            }

            buf.push_str("]),\n");
        }

        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end coverage

    buf.push_str("}\n"); // end impl Family

    Ok(())
}

pub fn wrt_fle_font(fnts: &[Arc<RwLock<Fnt>>], buf: &mut String) {
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
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
#[repr(u32)] // for `mem::transmute`
pub enum Font {
"#);
    let mut id: u16 = 0;
    let mut fam_prv_o: Option<Arc<RwLock<Fam>>> = None;
    for fnt in fnts.iter() {
        if let Some(fam_prv) = fam_prv_o {
            if fnt.read().unwrap().fam.read().unwrap().variant == fam_prv.read().unwrap().variant {
                id += 1;
            } else {
                id = 0;
            }
        }

        buf.push_str(&format!(
            "    /// The [{}](https://fonts.google.com/specimen/{}) _{}_ font.\n",
            fnt.read().unwrap().fam.read().unwrap().name,
            &fnt.read()
                .unwrap()
                .fam
                .read()
                .unwrap()
                .name
                .replace(' ', "+"),
            fnt.read().unwrap().name_suffix()
        ));
        buf.push_str("    ///\n");
        buf.push_str(&format!(
            "    /// ![{}](https://rana.github.io/google-fonts/doc/imgs/{}.webp)\n",
            fnt.read().unwrap().name,
            fnt.read().unwrap().variant
        ));
        buf.push_str("    ///\n");
        buf.push_str(&format!(
            "    /// Designed by {}.\n",
            comma_and(&fnt.read().unwrap().fam.read().unwrap().meta.designers)
        ));
        buf.push_str(&cfg_feature("    ", fnt.read().unwrap().features()));
        if id == 0 {
            buf.push_str(&format!(
                "    {} = Family::{} as u32,\n",
                fnt.read().unwrap().variant,
                fnt.read().unwrap().fam.read().unwrap().variant
            ));
        } else {
            buf.push_str(&format!(
                "    {} = {} + Family::{} as u32,\n",
                fnt.read().unwrap().variant,
                id,
                fnt.read().unwrap().fam.read().unwrap().variant
            ));
        }

        fam_prv_o = Some(fnt.read().unwrap().fam.clone());
    }
    buf.push_str("}\n"); // end enum Font

    buf.push_str(
        r#"
impl Font {
    /// Returns the _id_ for the [`Font`].
    pub fn id(&self) -> u32 {
        *self as u32
    }

    /// Returns the [`Family`].
    pub fn family(&self) -> Family {
        Family::from_id((self.id() / ID_INCREMENT) * ID_INCREMENT)
    }

    /// Returns the index of the font file for the [`Family`].
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
    /// Get TTF font data from the network.
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

    /// Get TTF font data and store locally.
    pub fn get_with_cache(&self) -> Result<Vec<u8>, FontError> {
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
    buf.push_str("    /// Transforms an _id_ into a [`Font`].\n");
    buf.push_str("    pub(crate) fn from_id(id: u32) -> Self {\n");
    buf.push_str("        unsafe { std::mem::transmute(id) }\n");
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
        buf.push_str(&cfg_feature("            ", fnt.read().unwrap().features()));
        buf.push_str(&format!(
            "            {}::{} => {}::{},\n",
            FONT,
            fnt.read().unwrap().variant,
            CATEGORY,
            fnt.read()
                .unwrap()
                .fam
                .read()
                .unwrap()
                .cat
                .read()
                .unwrap()
                .variant
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

pub fn wrt_fle_category(cats: &[Arc<RwLock<Cat>>], buf: &mut String) {
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
/// 
/// A font has one category.
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
"#);
    buf.push_str(&format!("pub enum {} {{\n", CATEGORY));
    for cat in cats.iter() {
        buf.push_str(&format!(
            "    /// The _{}_ font category.\n",
            cat.read().unwrap().name
        ));
        buf.push_str(&format!("    {},\n", cat.read().unwrap().variant));
    }
    buf.push_str("}\n"); // end enum Category

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
    for cat in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => \"{}\".into(),\n",
            CATEGORY,
            cat.read().unwrap().variant,
            cat.read().unwrap().name
        ));
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n");

    // Write `families`.
    buf.push('\n');
    buf.push_str("    /// Returns families for the [`Category`].\n");
    buf.push_str(&format!(
        "    pub fn families(&self) -> Vec<{}> {{\n",
        FAMILY
    ));
    buf.push_str("        match self {\n");
    for cat in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            CATEGORY,
            cat.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");
        for fam in cat.read().unwrap().fams.iter() {
            buf.push_str(&cfg_feature(
                "                    ",
                fam.read().unwrap().features(),
            ));
            buf.push_str(&format!(
                "                    {}::{},\n",
                FAMILY,
                fam.read().unwrap().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `families`

    // Write `fonts`.
    buf.push('\n');
    buf.push_str("    /// Returns fonts for the [`Category`].\n");
    buf.push_str(&format!("    pub fn fonts(&self) -> Vec<{}> {{\n", FONT));
    buf.push_str("        match self {\n");
    for cat in cats.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            CATEGORY,
            cat.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");
        for fnt in cat.read().unwrap().fnts.iter() {
            buf.push_str(&cfg_feature(
                "                    ",
                fnt.read().unwrap().features(),
            ));
            buf.push_str(&format!(
                "                    {}::{},\n",
                FONT,
                fnt.read().unwrap().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `fonts`

    buf.push_str("}\n"); // end impl Family
}

pub fn wrt_fle_subset(subs: &[Arc<RwLock<Sub>>], buf: &mut String) {
    // Write enum.
    // pub enum Subset {
    //     Latin,
    // }
    buf.push_str(r#"
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::family::Family;
use crate::font::Font;

/// An _enumeration_ of font subsets.
/// 
/// A font has one or more subsets.
#[derive(Debug, Display, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
"#);
    buf.push_str(&format!("pub enum {} {{\n", SUBSET));
    for sub in subs.iter() {
        buf.push_str(&format!(
            "    /// The _{}_ font subset.\n",
            sub.read().unwrap().name
        ));
        buf.push_str(&format!("    {},\n", sub.read().unwrap().variant));
    }
    buf.push_str("}\n"); // end enum Subset

    // Write impl Subset.
    buf.push('\n');
    buf.push_str(&format!("impl {} {{\n", SUBSET));

    // Write `families`.
    buf.push('\n');
    buf.push_str("    /// Returns families for the [`Subset`].\n");
    buf.push_str(&format!(
        "    pub fn families(&self) -> Vec<{}> {{\n",
        FAMILY
    ));
    buf.push_str("        match self {\n");
    for sub in subs.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            SUBSET,
            sub.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");
        for fam in sub.read().unwrap().fams.iter() {
            buf.push_str(&cfg_feature(
                "                    ",
                fam.read().unwrap().features(),
            ));
            buf.push_str(&format!(
                "                    {}::{},\n",
                FAMILY,
                fam.read().unwrap().variant
            ));
        }
        buf.push_str("                ]\n");
        buf.push_str("            }\n");
    }
    buf.push_str("        }\n");
    buf.push_str("    }\n"); // end `families`

    // Write `fonts`.
    buf.push('\n');
    buf.push_str("    /// Returns fonts for the [`Subset`].\n");
    buf.push_str(&format!("    pub fn fonts(&self) -> Vec<{}> {{\n", FONT));
    buf.push_str("        match self {\n");
    for sub in subs.iter() {
        buf.push_str(&format!(
            "            {}::{} => {{\n",
            SUBSET,
            sub.read().unwrap().variant
        ));
        buf.push_str("                vec![\n");
        for fnt in sub.read().unwrap().fnts.iter() {
            buf.push_str(&cfg_feature(
                "                    ",
                fnt.read().unwrap().features(),
            ));
            buf.push_str(&format!(
                "                    {}::{},\n",
                FONT,
                fnt.read().unwrap().variant
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

pub fn wrt_fle_lib(fnts: &[Arc<RwLock<Fnt>>], buf: &mut String) {
    buf.push_str(
        r#"
pub mod category;
pub mod error;
pub mod family;
pub mod font;
pub mod subset;
pub use crate::category::*;
pub use crate::error::*;
pub use crate::family::*;
pub use crate::font::*;
pub use crate::subset::*;
"#,
    );

    // Write individual font functions.
    for fnt in fnts.iter() {
        buf.push('\n');
        buf.push_str(&format!(
            "/// Get font data for the [{}](https://fonts.google.com/specimen/{}) _{}_ font.\n",
            fnt.read().unwrap().fam.read().unwrap().name,
            fnt.read()
                .unwrap()
                .fam
                .read()
                .unwrap()
                .name
                .replace(' ', "+"),
            fnt.read().unwrap().name_suffix(),
        ));
        buf.push_str("///\n");
        buf.push_str("/// Loaded from the network and cached to disk.\n");
        buf.push_str("///\n");
        buf.push_str(&format!(
            "/// ![{}](https://rana.github.io/google-fonts/doc/imgs/{}.webp)\n",
            fnt.read().unwrap().name,
            fnt.read().unwrap().variant
        ));
        buf.push_str("///\n");
        buf.push_str(&format!(
            "/// Designed by {}.\n",
            comma_and(&fnt.read().unwrap().fam.read().unwrap().meta.designers)
        ));
        buf.push_str(&cfg_feature("", fnt.read().unwrap().features()));
        buf.push_str(&format!(
            "pub fn {}() -> Result<Vec<u8>, FontError> {{\n",
            fnt.read().unwrap().fn_name()
        ));
        buf.push_str(&format!(
            "    {}::{}.get_with_cache()\n",
            FONT,
            fnt.read().unwrap().variant
        ));
        buf.push_str("}\n");
    }

    // Write `tests` module.
    buf.push('\n');
    buf.push_str("#[cfg(test)]\n");
    buf.push_str("mod tests {\n");
    buf.push_str("    use super::*;\n");
    buf.push_str("    use ttf_parser::Face;\n");

    // Test Family-Font id casting.
    buf.push('\n');
    buf.push_str("    #[test]\n");
    buf.push_str("    #[cfg(feature = \"static\")]\n");
    buf.push_str("    fn test_cast_family_font() {\n");
    buf.push_str("        let fam = Family::ABeeZee;\n");
    buf.push_str("        let fnt = Font::ABeeZeeRegular;\n");
    buf.push_str("        assert_eq!(fam, fnt.family());\n");
    buf.push_str("        assert_eq!(fnt, fam.font());\n");
    buf.push_str("    }\n");

    // Test getting each font's data.
    for fnt in fnts.iter() {
        buf.push('\n');
        buf.push_str("    #[test]\n");
        buf.push_str(&cfg_feature("    ", fnt.read().unwrap().features()));
        buf.push_str(&format!(
            "    fn test_{}() {{\n",
            fnt.read().unwrap().fn_name()
        ));
        buf.push_str(&format!(
            "        let result = {}();\n",
            fnt.read().unwrap().fn_name()
        ));
        buf.push_str("        assert!(result.is_ok());\n");
        buf.push_str("        let font_data = result.unwrap();\n");
        buf.push_str("        let result2 = Face::parse(&font_data, 0);\n");
        buf.push_str("        assert!(result2.is_ok());\n");
        buf.push_str("    }\n");
    }
    buf.push_str("}\n"); // end mod tests
}

pub fn wrt_fle_imgs(fnts: &[Arc<RwLock<Fnt>>], cli: &Client) -> Result<()> {
    let pth = PathBuf::from("../doc/imgs");

    // Re-create image directory.
    // Ensure we don't keep images for deleted fonts.
    if pth.exists() {
        fs::remove_dir_all(&pth)?;
    }
    fs::create_dir_all(&pth)?;

    fnts.par_iter().for_each(|fnt| {
        let mut attempt = 0;
        loop {
            attempt += 1;
            match process_font(fnt, cli, pth.clone()) {
                Ok(_) => break, // If successful, break the loop
                Err(e) => {
                    if attempt >= MAX_RETRIES {
                        eprintln!(
                            "Failed to process font {} after {} attempts: {}",
                            fnt.read().unwrap().name,
                            attempt,
                            e
                        );
                        break;
                    }
                    eprintln!(
                        "Error processing font {} (attempt {}): {}. Retrying...",
                        fnt.read().unwrap().name,
                        attempt,
                        e
                    );
                    thread::sleep(RETRY_DELAY); // Sleep before retrying
                }
            }
        }
    });

    Ok(())
}

fn process_font(fnt: &Arc<RwLock<Fnt>>, cli: &Client, mut pth: PathBuf) -> Result<()> {
    // Get font.
    let mgr = FontMgr::new();
    let mut paint1 = Paint::default();
    paint1
        .set_color(Color::from_rgb(255, 255, 255))
        .set_anti_alias(false);
    let fnt_dat = fnt.read().unwrap().get(cli)?;
    let face = mgr.new_from_data(&fnt_dat, None).unwrap();
    let font = &Font::from_typeface(face, 18.0);

    // Get sample text. Written for each language.
    let txt = fnt.read().unwrap().get_sampletext(cli)?;

    // Measure font.
    let (_, mut rect) = font.measure_str(&txt, Some(&paint1));
    let mrg: f32 = 4.0;

    // Add margin to image.
    rect.left -= mrg;
    rect.right += mrg;
    rect.bottom += mrg;
    rect.top -= mrg;

    // Draw image.
    let size = (rect.size().width as i32, rect.size().height as i32);
    let mut surface = surfaces::raster_n32_premul(size).unwrap();
    let canvas = surface.canvas();
    canvas.draw_str(&txt, (mrg, rect.size().height - rect.bottom), font, &paint1);
    let image = surface.image_snapshot();
    let data = image
        .encode(
            &mut surface.direct_context(),
            EncodedImageFormat::WEBP,
            Some(0),
        )
        .unwrap();

    // Save file.
    pth.push(&fnt.read().unwrap().variant);
    pth.set_extension("webp");
    fs::write(&pth, data.as_bytes())?;

    Ok(())
}

pub fn wrt_fle_cargo_toml(dir_pth: &str) -> Result<()> {
    let mut pth = PathBuf::from(dir_pth);
    pth.pop();
    pth.push("Cargo");
    pth.set_extension("toml");

    // Read the cargo file
    let mut man = cargo_toml::Manifest::from_path(&pth)?;

    // Clear any previous features
    man.features.clear();

    // Add features.
    man.features.insert("default".into(), vec![FULL.into()]);
    man.features
        .insert(FULL.into(), vec![VARIABLE.into(), STATIC.into()]);
    man.features.insert(VARIABLE.into(), vec![]);
    man.features.insert(STATIC.into(), vec![]);

    // Serialize the mutated manifest back to TOML format
    let toml_string = toml::ser::to_string(&man)?;

    // Open the Cargo.toml file in write mode
    let mut file = File::create(&pth)?;

    // Write the serialized TOML string to the file
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

pub fn cfg_feature(indent: &str, features: Vec<String>) -> String {
    if features.len() == 1 {
        format!("{}#[cfg(feature = \"{}\")]\n", indent, features[0])
    } else {
        // #[cfg(any(feature = "feature1", feature = "feature2"))]
        let mut buf = String::with_capacity(1_024);
        buf.push_str(indent);
        buf.push_str("#[cfg(any(");
        for (idx, feature) in features.iter().enumerate() {
            if idx != 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format!("feature = \"{}\"", feature));
        }
        buf.push_str("))]\n");
        buf
    }
}

/// Enumerate items in a sentence.
///
/// For example, "thing1, thing2, and thing3".
pub fn comma_and(vals: &[String]) -> String {
    match vals.len() {
        0 => "".into(),
        1 => format!("_{}_", vals[0]),
        2 => format!("_{}_ and _{}_", vals[0], vals[1]),
        _ => {
            let cap: usize = vals.iter().map(|o| o.len()).sum();
            let mut buf = String::with_capacity(cap + ((vals.len() - 1) * 2) + 4);
            let idx_lst = vals.len() - 1;
            for (idx, val) in vals.iter().enumerate() {
                if idx != 0 {
                    if idx != idx_lst {
                        buf.push_str(", ");
                    } else {
                        buf.push_str(", and ");
                    }
                }
                buf.push_str(&format!("_{}_", val));
            }

            buf
        }
    }
}

pub fn rc<T>(v: T) -> Arc<RwLock<T>> {
    Arc::new(RwLock::new(v))
}

pub fn sub_variant(name: String) -> String {
    name.to_title_case().replace(' ', "")
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

/// Get a metadata list for font families.
pub fn get_family_metadata_list(cli: &Client) -> Result<Vec<FamilyMetadata>> {
    let txt = cli
        .get("https://fonts.google.com/metadata/fonts")
        .send()?
        .text()?;
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

pub fn cache_dir() -> PathBuf {
    CACHE_DIR.clone()
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

#[derive(Debug, Clone)]
pub struct Fam {
    pub id: u32,
    pub name: String,
    pub variant: String,
    pub cat: Arc<RwLock<Cat>>,
    pub meta: FamilyMetadata,
    pub fnts: Vec<Arc<RwLock<Fnt>>>,
    pub subs: Vec<Arc<RwLock<Sub>>>,
}
#[derive(Debug, Clone)]
pub struct Fnt {
    pub id: u32,
    pub name: String,
    pub variant: String,
    pub fam: Arc<RwLock<Fam>>,
    pub subs: Vec<Arc<RwLock<Sub>>>,
}

#[derive(Debug, Clone)]
pub struct Cat {
    pub name: String,
    pub variant: String,
    pub fams: Vec<Arc<RwLock<Fam>>>,
    pub fnts: Vec<Arc<RwLock<Fnt>>>,
}

#[derive(Debug, Clone)]
pub struct Sub {
    pub name: String,
    pub variant: String,
    pub fams: Vec<Arc<RwLock<Fam>>>,
    pub fnts: Vec<Arc<RwLock<Fnt>>>,
}

impl Fam {
    pub fn features(&self) -> Vec<String> {
        let mut ret = Vec::new();
        if self.fnts.iter().any(|o| o.read().unwrap().is_variable()) {
            ret.push(VARIABLE.into());
        }
        if self.fnts.iter().any(|o| o.read().unwrap().is_static()) {
            ret.push(STATIC.into());
        }
        ret
    }

    /// Get a file list for the font family.
    pub fn get_file_list(&self, cli: &Client) -> Result<Manifest> {
        // Create file path.
        let mut pth = cache_dir();
        pth.push(&format!("{}_file_list", &self.variant));
        pth.set_extension("json");

        // Check if the file already exists.
        if pth.exists() {
            // Load the file.
            let fle = File::open(pth)?;
            let rdr = BufReader::new(fle);

            // Deserialize the JSON into a struct.
            let ret: FamilyFileList = serde_json::from_reader(rdr)?;
            return Ok(ret.manifest);
        }

        let txt = cli
            .get("https://fonts.google.com/download/list")
            .query(&[("family", &self.name)])
            .send()?
            .text()?;

        // Trim leading excess characters
        // to allow deserialization.
        //  ")]}'\n{\n
        let mut txt: &str = txt.as_ref();
        if let Some(idx) = txt.find('{') {
            if idx != 0 {
                txt = &txt[idx..];
            }
        }

        // Write the data to disk for caching.
        // eprintln!("writing {:?}", &pth);
        fs::write(pth, txt)?;

        // Deserialize JSON to struct.
        let ret: FamilyFileList = serde_json::from_str(txt)?;

        Ok(ret.manifest)
    }

    /// Get metadata detail for the font family.
    pub fn get_metadata_detail(&self, cli: &Client) -> Result<FamilyMetadataDetail> {
        // Create file path.
        let mut pth = cache_dir();
        pth.push(&format!("{}_meta", &self.variant));
        pth.set_extension("json");

        // Check if the file already exists.
        if pth.exists() {
            // Load the file.
            let fle = File::open(pth)?;
            let rdr = BufReader::new(fle);

            // Deserialize the JSON into a struct.
            let ret: FamilyMetadataDetail = serde_json::from_reader(rdr)?;
            return Ok(ret);
        }

        let url = format!(
            "https://fonts.google.com/metadata/fonts/{}",
            self.name.replace(' ', "+")
        );
        let txt = cli.get(url).send()?.text()?;

        // Trim leading excess characters
        // to allow deserialization.
        //  ")]}'\n{\n
        let mut txt: &str = txt.as_ref();
        if let Some(idx) = txt.find('{') {
            if idx != 0 {
                txt = &txt[idx..];
            }
        }

        // Write the data to disk for caching.
        // eprintln!("writing {:?}", &pth);
        fs::write(pth, txt)?;

        // Deserialize JSON to struct.
        let ret: FamilyMetadataDetail = serde_json::from_str(txt)?;

        Ok(ret)
    }
}

impl Fnt {
    /// Get the font name suffix. For example, _regular_.
    pub fn name_suffix(&self) -> String {
        self.name
            .replace(&self.fam.read().unwrap().variant, "")
            .trim()
            .to_title_case()
            .to_lowercase()
    }

    // Get the font function name.
    pub fn fn_name(&self) -> String {
        let suffix = self
            .variant
            .replace(&self.fam.read().unwrap().variant, "")
            .trim()
            .to_title_case()
            .to_lowercase();
        format!(
            "{} {}",
            self.fam.read().unwrap().name.to_lowercase(),
            suffix
        )
        .replace(' ', "_")
    }

    /// Get sample text from network or cache.
    pub fn get_sampletext(&self, cli: &Client) -> Result<String> {
        // Create file path.
        let mut pth = cache_dir();
        pth.push(format!("{}_sampletext", &self.variant));
        pth.set_extension("json");

        // Load cached file if exists.
        if pth.exists() {
            let fle = File::open(pth)?;
            let rdr = BufReader::new(fle);
            let ret: FamilySampleText = serde_json::from_reader(rdr)?;
            return Ok(ret.sample_text.txt());
        }

        // Get the sample text from the network.
        let txt = cli
            .get("https://fonts.google.com/sampletext")
            .query(&[
                ("family", self.fam.read().unwrap().name.as_str()),
                // ("paragraphOnly", "true"),
            ])
            .send()?
            .text()?;

        // Trim leading excess characters
        // to allow deserialization.
        //  ")]}'\n{\n
        let mut txt: &str = txt.as_ref();
        if let Some(idx) = txt.find('{') {
            if idx != 0 {
                txt = &txt[idx..];
            }
        }

        // Write the data to disk for caching.
        // eprintln!("writing {:?}", &pth);
        fs::write(pth, txt)?;

        // Deserialize JSON to struct.
        let ret: FamilySampleText = serde_json::from_str(txt)?;

        Ok(ret.sample_text.txt())
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

        let file_refs = self.fam.read().unwrap().get_file_list(cli)?.file_refs;

        // Get the font index.
        let (idx, _) = self
            .fam
            .read()
            .unwrap()
            .fnts
            .iter()
            .enumerate()
            .find(|(_, o)| o.read().unwrap().name == self.name)
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

    pub fn is_variable(&self) -> bool {
        self.name.contains("Variable")
    }

    pub fn is_static(&self) -> bool {
        !self.is_variable()
    }

    pub fn features(&self) -> Vec<String> {
        if self.is_variable() {
            vec![VARIABLE.into()]
        } else {
            vec![STATIC.into()]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMetadataDetail {
    family: String,
    display_name: Option<String>,
    coverage: HashMap<String, String>,
    description: String,
    languages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlyphGroup {
    name: String,
    glyphs: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleText {
    masthead_full: String,
    masthead_partial: String,
    styles: String,
    tester: String,
    poster_sm: Option<String>,
    poster_md: Option<String>,
    poster_lg: Option<String>,
    languages: Vec<String>,
}
impl SampleText {
    pub fn txt(&self) -> String {
        self.styles.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilySampleText {
    glyph_groups: Vec<GlyphGroup>,
    sample_text: SampleText,
}

lazy_static! {
    static ref CACHE_DIR: PathBuf = {
        let mut pth = dirs::cache_dir().expect("Failed to get cache directory");
        pth.push("google-fonts");
        pth.push("gen");

        // Create the directory and any missing parent directories if it doesn't exist.
        if !pth.exists() {
            fs::create_dir_all(&pth).expect("Failed to create cache directory");
        }

        pth
    };
}
