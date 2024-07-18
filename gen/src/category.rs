
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
                    Family::ADLaMDisplay,
                    Family::Aboreto,
                    Family::AbrilFatface,
                ]
            }
            Category::SansSerif => {
                vec![
                    Family::ABeeZee,
                    Family::AROneSans,
                    Family::Abel,
                    Family::Aclonica,
                    Family::Acme,
                    Family::Actor,
                ]
            }
            Category::Serif => {
                vec![
                    Family::AbhayaLibre,
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
                    Font::ADLaMDisplayRegular,
                    Font::AboretoRegular,
                    Font::AbrilFatfaceRegular,
                ]
            }
            Category::SansSerif => {
                vec![
                    Font::ABeeZeeRegular,
                    Font::ABeeZeeItalic,
                    Font::AROneSansRegular,
                    Font::AROneSansMedium,
                    Font::AROneSansSemiBold,
                    Font::AROneSansBold,
                    Font::AROneSansVariable,
                    Font::AbelRegular,
                    Font::AclonicaRegular,
                    Font::AcmeRegular,
                    Font::ActorRegular,
                ]
            }
            Category::Serif => {
                vec![
                    Font::AbhayaLibreRegular,
                    Font::AbhayaLibreMedium,
                    Font::AbhayaLibreSemiBold,
                    Font::AbhayaLibreBold,
                    Font::AbhayaLibreExtraBold,
                    Font::AbyssinicaSILRegular,
                ]
            }
        }
    }
}
