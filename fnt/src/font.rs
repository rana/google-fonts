
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
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _regular_ font.
    ///
    /// ![ABeeZee Regular](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeRegular.svg)
    #[cfg(feature = "static")]
    ABeeZeeRegular = Family::ABeeZee as isize,
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _italic_ font.
    ///
    /// ![ABeeZee Italic](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeItalic.svg)
    #[cfg(feature = "static")]
    ABeeZeeItalic = 1 + Family::ABeeZee as isize,
    /// The [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) _regular_ font.
    ///
    /// ![ADLaMDisplay Regular](https://rana.github.io/google-fonts/doc/imgs/ADLaMDisplayRegular.svg)
    #[cfg(feature = "static")]
    ADLaMDisplayRegular = Family::ADLaMDisplay as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _regular_ font.
    ///
    /// ![AROneSans Regular](https://rana.github.io/google-fonts/doc/imgs/AROneSansRegular.svg)
    #[cfg(feature = "static")]
    AROneSansRegular = Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _medium_ font.
    ///
    /// ![AROneSans Medium](https://rana.github.io/google-fonts/doc/imgs/AROneSansMedium.svg)
    #[cfg(feature = "static")]
    AROneSansMedium = 1 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _semi bold_ font.
    ///
    /// ![AROneSans SemiBold](https://rana.github.io/google-fonts/doc/imgs/AROneSansSemiBold.svg)
    #[cfg(feature = "static")]
    AROneSansSemiBold = 2 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _bold_ font.
    ///
    /// ![AROneSans Bold](https://rana.github.io/google-fonts/doc/imgs/AROneSansBold.svg)
    #[cfg(feature = "static")]
    AROneSansBold = 3 + Family::AROneSans as isize,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _variable_ font.
    ///
    /// ![AROneSans Variable](https://rana.github.io/google-fonts/doc/imgs/AROneSansVariable.svg)
    #[cfg(feature = "variable")]
    AROneSansVariable = 4 + Family::AROneSans as isize,
    /// The [Abel](https://fonts.google.com/specimen/Abel) _regular_ font.
    ///
    /// ![Abel Regular](https://rana.github.io/google-fonts/doc/imgs/AbelRegular.svg)
    #[cfg(feature = "static")]
    AbelRegular = Family::Abel as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _regular_ font.
    ///
    /// ![AbhayaLibre Regular](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreRegular.svg)
    #[cfg(feature = "static")]
    AbhayaLibreRegular = Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _medium_ font.
    ///
    /// ![AbhayaLibre Medium](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreMedium.svg)
    #[cfg(feature = "static")]
    AbhayaLibreMedium = 1 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _semi bold_ font.
    ///
    /// ![AbhayaLibre SemiBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreSemiBold.svg)
    #[cfg(feature = "static")]
    AbhayaLibreSemiBold = 2 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _bold_ font.
    ///
    /// ![AbhayaLibre Bold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreBold.svg)
    #[cfg(feature = "static")]
    AbhayaLibreBold = 3 + Family::AbhayaLibre as isize,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _extra bold_ font.
    ///
    /// ![AbhayaLibre ExtraBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreExtraBold.svg)
    #[cfg(feature = "static")]
    AbhayaLibreExtraBold = 4 + Family::AbhayaLibre as isize,
    /// The [Aboreto](https://fonts.google.com/specimen/Aboreto) _regular_ font.
    ///
    /// ![Aboreto Regular](https://rana.github.io/google-fonts/doc/imgs/AboretoRegular.svg)
    #[cfg(feature = "static")]
    AboretoRegular = Family::Aboreto as isize,
    /// The [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) _regular_ font.
    ///
    /// ![AbrilFatface Regular](https://rana.github.io/google-fonts/doc/imgs/AbrilFatfaceRegular.svg)
    #[cfg(feature = "static")]
    AbrilFatfaceRegular = Family::AbrilFatface as isize,
    /// The [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) _regular_ font.
    ///
    /// ![AbyssinicaSIL Regular](https://rana.github.io/google-fonts/doc/imgs/AbyssinicaSILRegular.svg)
    #[cfg(feature = "static")]
    AbyssinicaSILRegular = Family::AbyssinicaSIL as isize,
    /// The [Aclonica](https://fonts.google.com/specimen/Aclonica) _regular_ font.
    ///
    /// ![Aclonica Regular](https://rana.github.io/google-fonts/doc/imgs/AclonicaRegular.svg)
    #[cfg(feature = "static")]
    AclonicaRegular = Family::Aclonica as isize,
    /// The [Acme](https://fonts.google.com/specimen/Acme) _regular_ font.
    ///
    /// ![Acme Regular](https://rana.github.io/google-fonts/doc/imgs/AcmeRegular.svg)
    #[cfg(feature = "static")]
    AcmeRegular = Family::Acme as isize,
    /// The [Actor](https://fonts.google.com/specimen/Actor) _regular_ font.
    ///
    /// ![Actor Regular](https://rana.github.io/google-fonts/doc/imgs/ActorRegular.svg)
    #[cfg(feature = "static")]
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
            #[cfg(feature = "static")]
            0 => Some(Font::ABeeZeeRegular),
            #[cfg(feature = "static")]
            1 => Some(Font::ABeeZeeItalic),
            #[cfg(feature = "static")]
            1000 => Some(Font::ADLaMDisplayRegular),
            #[cfg(feature = "static")]
            2000 => Some(Font::AROneSansRegular),
            #[cfg(feature = "static")]
            2001 => Some(Font::AROneSansMedium),
            #[cfg(feature = "static")]
            2002 => Some(Font::AROneSansSemiBold),
            #[cfg(feature = "static")]
            2003 => Some(Font::AROneSansBold),
            #[cfg(feature = "variable")]
            2004 => Some(Font::AROneSansVariable),
            #[cfg(feature = "static")]
            3000 => Some(Font::AbelRegular),
            #[cfg(feature = "static")]
            4000 => Some(Font::AbhayaLibreRegular),
            #[cfg(feature = "static")]
            4001 => Some(Font::AbhayaLibreMedium),
            #[cfg(feature = "static")]
            4002 => Some(Font::AbhayaLibreSemiBold),
            #[cfg(feature = "static")]
            4003 => Some(Font::AbhayaLibreBold),
            #[cfg(feature = "static")]
            4004 => Some(Font::AbhayaLibreExtraBold),
            #[cfg(feature = "static")]
            5000 => Some(Font::AboretoRegular),
            #[cfg(feature = "static")]
            6000 => Some(Font::AbrilFatfaceRegular),
            #[cfg(feature = "static")]
            7000 => Some(Font::AbyssinicaSILRegular),
            #[cfg(feature = "static")]
            8000 => Some(Font::AclonicaRegular),
            #[cfg(feature = "static")]
            9000 => Some(Font::AcmeRegular),
            #[cfg(feature = "static")]
            10000 => Some(Font::ActorRegular),
            _ => None,
        }
    }
    /// Returns the font [`Category`].
    pub fn category(&self) -> Category {
        match self {
            #[cfg(feature = "static")]
            Font::ABeeZeeRegular => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::ABeeZeeItalic => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::ADLaMDisplayRegular => Category::Display,
            #[cfg(feature = "static")]
            Font::AROneSansRegular => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AROneSansMedium => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AROneSansSemiBold => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AROneSansBold => Category::SansSerif,
            #[cfg(feature = "variable")]
            Font::AROneSansVariable => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AbelRegular => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AbhayaLibreRegular => Category::Serif,
            #[cfg(feature = "static")]
            Font::AbhayaLibreMedium => Category::Serif,
            #[cfg(feature = "static")]
            Font::AbhayaLibreSemiBold => Category::Serif,
            #[cfg(feature = "static")]
            Font::AbhayaLibreBold => Category::Serif,
            #[cfg(feature = "static")]
            Font::AbhayaLibreExtraBold => Category::Serif,
            #[cfg(feature = "static")]
            Font::AboretoRegular => Category::Display,
            #[cfg(feature = "static")]
            Font::AbrilFatfaceRegular => Category::Display,
            #[cfg(feature = "static")]
            Font::AbyssinicaSILRegular => Category::Serif,
            #[cfg(feature = "static")]
            Font::AclonicaRegular => Category::SansSerif,
            #[cfg(feature = "static")]
            Font::AcmeRegular => Category::SansSerif,
            #[cfg(feature = "static")]
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
    