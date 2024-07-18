use crate::category::Category;
use crate::error::{FontError, StringError};
use crate::family::{Family, ID_INCREMENT};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString};

/// An _enumeration_ of [Google fonts](https://fonts.google.com).
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    EnumCount,
    EnumIter,
    EnumString,
    AsRefStr,
)]
pub enum Font {
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _regular_ font.
    ///
    /// ![ABeeZee Regular](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeRegular.svg)
    /// 
    /// ```html
    /// <link rel="stylesheet" href="[https://fonts.googleapis.com/css2?family=Open+Sans:wght@300;400&display=swap](https://fonts.googleapis.com/css2?family=Open+Sans:wght@300;400&display=swap)">
    /// ```
    ///
    /// This would allow you to style text using the `Open Sans` webfont.
    ///
    /// ```html
    /// <p style="font-family: 'Open Sans', sans-serif;">This text uses the Open Sans webfont.</p>
    ABeeZeeRegular = Family::ABeeZee as isize,
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _italic_ font.
    ///
    /// ![ABeeZee Italic](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeItalic.svg)
    ABeeZeeItalic = 1 + Family::ABeeZee as isize,
    /// The [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) _regular_ font.
    ///
    /// ![ADLaMDisplay Regular](https://rana.github.io/google-fonts/doc/imgs/ADLaMDisplayRegular.svg)
    ADLaMDisplayRegular = Family::ADLaMDisplay as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _regular_ font.
    ///
    /// ![AROneSans Regular](https://rana.github.io/google-fonts/doc/imgs/AROneSansRegular.svg)
    AROneSansRegular = Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _medium_ font.
    ///
    /// ![AROneSans Medium](https://rana.github.io/google-fonts/doc/imgs/AROneSansMedium.svg)
    AROneSansMedium = 1 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _semi bold_ font.
    ///
    /// ![AROneSans SemiBold](https://rana.github.io/google-fonts/doc/imgs/AROneSansSemiBold.svg)
    AROneSansSemiBold = 2 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _bold_ font.
    ///
    /// ![AROneSans Bold](https://rana.github.io/google-fonts/doc/imgs/AROneSansBold.svg)
    AROneSansBold = 3 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _variable_ font.
    ///
    /// ![AROneSans Variable](https://rana.github.io/google-fonts/doc/imgs/AROneSansVariable.svg)
    AROneSansVariable = 4 + Family::AROneSans as isize,
    /// The [Abel](https://fonts.google.com/specimen/Abel) _regular_ font.
    ///
    /// ![Abel Regular](https://rana.github.io/google-fonts/doc/imgs/AbelRegular.svg)
    AbelRegular = Family::Abel as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _regular_ font.
    ///
    /// ![AbhayaLibre Regular](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreRegular.svg)
    AbhayaLibreRegular = Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _medium_ font.
    ///
    /// ![AbhayaLibre Medium](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreMedium.svg)
    AbhayaLibreMedium = 1 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _semi bold_ font.
    ///
    /// ![AbhayaLibre SemiBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreSemiBold.svg)
    AbhayaLibreSemiBold = 2 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _bold_ font.
    ///
    /// ![AbhayaLibre Bold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreBold.svg)
    AbhayaLibreBold = 3 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _extra bold_ font.
    ///
    /// ![AbhayaLibre ExtraBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreExtraBold.svg)
    AbhayaLibreExtraBold = 4 + Family::AbhayaLibre as isize,
    /// The [Aboreto](https://fonts.google.com/specimen/Aboreto) _regular_ font.
    ///
    /// ![Aboreto Regular](https://rana.github.io/google-fonts/doc/imgs/AboretoRegular.svg)
    AboretoRegular = Family::Aboreto as isize,
    /// The [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) _regular_ font.
    ///
    /// ![AbrilFatface Regular](https://rana.github.io/google-fonts/doc/imgs/AbrilFatfaceRegular.svg)
    AbrilFatfaceRegular = Family::AbrilFatface as isize,
    /// The [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) _regular_ font.
    ///
    /// ![AbyssinicaSIL Regular](https://rana.github.io/google-fonts/doc/imgs/AbyssinicaSILRegular.svg)
    AbyssinicaSILRegular = Family::AbyssinicaSIL as isize,
    /// The [Aclonica](https://fonts.google.com/specimen/Aclonica) _regular_ font.
    ///
    /// ![Aclonica Regular](https://rana.github.io/google-fonts/doc/imgs/AclonicaRegular.svg)
    AclonicaRegular = Family::Aclonica as isize,
    /// The [Acme](https://fonts.google.com/specimen/Acme) _regular_ font.
    ///
    /// ![Acme Regular](https://rana.github.io/google-fonts/doc/imgs/AcmeRegular.svg)
    AcmeRegular = Family::Acme as isize,
    /// The [Actor](https://fonts.google.com/specimen/Actor) _regular_ font.
    ///
    /// ![Actor Regular](https://rana.github.io/google-fonts/doc/imgs/ActorRegular.svg)
    ActorRegular = Family::Actor as isize,
}

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

    /// Converts an `isize` to a [`Font`].
    pub fn from_id(id: isize) -> Option<Self> {
        match id {
            0 => Some(Font::ABeeZeeRegular),
            1 => Some(Font::ABeeZeeItalic),
            1000 => Some(Font::ADLaMDisplayRegular),
            2000 => Some(Font::AROneSansRegular),
            2001 => Some(Font::AROneSansMedium),
            2002 => Some(Font::AROneSansSemiBold),
            2003 => Some(Font::AROneSansBold),
            2004 => Some(Font::AROneSansVariable),
            3000 => Some(Font::AbelRegular),
            4000 => Some(Font::AbhayaLibreRegular),
            4001 => Some(Font::AbhayaLibreMedium),
            4002 => Some(Font::AbhayaLibreSemiBold),
            4003 => Some(Font::AbhayaLibreBold),
            4004 => Some(Font::AbhayaLibreExtraBold),
            5000 => Some(Font::AboretoRegular),
            6000 => Some(Font::AbrilFatfaceRegular),
            7000 => Some(Font::AbyssinicaSILRegular),
            8000 => Some(Font::AclonicaRegular),
            9000 => Some(Font::AcmeRegular),
            10000 => Some(Font::ActorRegular),
            _ => None,
        }
    }
    /// Returns the font [`Category`].
    pub fn category(&self) -> Category {
        match self {
            Font::ABeeZeeRegular => Category::SansSerif,
            Font::ABeeZeeItalic => Category::SansSerif,
            Font::ADLaMDisplayRegular => Category::Display,
            Font::AROneSansRegular => Category::SansSerif,
            Font::AROneSansMedium => Category::SansSerif,
            Font::AROneSansSemiBold => Category::SansSerif,
            Font::AROneSansBold => Category::SansSerif,
            Font::AROneSansVariable => Category::SansSerif,
            Font::AbelRegular => Category::SansSerif,
            Font::AbhayaLibreRegular => Category::Serif,
            Font::AbhayaLibreMedium => Category::Serif,
            Font::AbhayaLibreSemiBold => Category::Serif,
            Font::AbhayaLibreBold => Category::Serif,
            Font::AbhayaLibreExtraBold => Category::Serif,
            Font::AboretoRegular => Category::Display,
            Font::AbrilFatfaceRegular => Category::Display,
            Font::AbyssinicaSILRegular => Category::Serif,
            Font::AclonicaRegular => Category::SansSerif,
            Font::AcmeRegular => Category::SansSerif,
            Font::ActorRegular => Category::SansSerif,
        }
    }
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
    url: String,
}
