
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::family::Family;
use crate::font::Font;

/// An _enumeration_ of font categories.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
pub enum Category {
    /// The _Display_ font category.
    Display,
    /// The _Sans Serif_ font category.
    SansSerif,
    /// The _Serif_ font category.
    Serif,
}

impl Category {

    /// The name of the font [`Category`] with spaces.
    pub fn name(&self) -> String {
        match self {
            Category::Display => "Display".into(),
            Category::SansSerif => "Sans Serif".into(),
            Category::Serif => "Serif".into(),
        }
    }

    /// Returns families within the [`Category`].
    pub fn families(&self) -> Vec<Family> {
        match self {
            Category::Display => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ADLaMDisplay,
                    #[cfg(feature = "static")]
                    Family::Aboreto,
                    #[cfg(feature = "static")]
                    Family::AbrilFatface,
                ]
            }
            Category::SansSerif => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ABeeZee,
                    #[cfg(any(feature = "variable", feature = "static"))]
                    Family::AROneSans,
                    #[cfg(feature = "static")]
                    Family::Abel,
                    #[cfg(feature = "static")]
                    Family::Aclonica,
                    #[cfg(feature = "static")]
                    Family::Acme,
                    #[cfg(feature = "static")]
                    Family::Actor,
                ]
            }
            Category::Serif => {
                vec![
                    #[cfg(feature = "static")]
                    Family::AbhayaLibre,
                    #[cfg(feature = "static")]
                    Family::AbyssinicaSIL,
                ]
            }
        }
    }

    /// Returns fonts within the [`Category`].
    pub fn fonts(&self) -> Vec<Font> {
        match self {
            Category::Display => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
                    #[cfg(feature = "static")]
                    Font::AboretoRegular,
                    #[cfg(feature = "static")]
                    Font::AbrilFatfaceRegular,
                ]
            }
            Category::SansSerif => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ABeeZeeRegular,
                    #[cfg(feature = "static")]
                    Font::ABeeZeeItalic,
                    #[cfg(feature = "static")]
                    Font::AROneSansRegular,
                    #[cfg(feature = "static")]
                    Font::AROneSansMedium,
                    #[cfg(feature = "static")]
                    Font::AROneSansSemiBold,
                    #[cfg(feature = "static")]
                    Font::AROneSansBold,
                    #[cfg(feature = "variable")]
                    Font::AROneSansVariable,
                    #[cfg(feature = "static")]
                    Font::AbelRegular,
                    #[cfg(feature = "static")]
                    Font::AclonicaRegular,
                    #[cfg(feature = "static")]
                    Font::AcmeRegular,
                    #[cfg(feature = "static")]
                    Font::ActorRegular,
                ]
            }
            Category::Serif => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AbhayaLibreRegular,
                    #[cfg(feature = "static")]
                    Font::AbhayaLibreMedium,
                    #[cfg(feature = "static")]
                    Font::AbhayaLibreSemiBold,
                    #[cfg(feature = "static")]
                    Font::AbhayaLibreBold,
                    #[cfg(feature = "static")]
                    Font::AbhayaLibreExtraBold,
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                ]
            }
        }
    }
}
