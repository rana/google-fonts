
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::family::Family;
use crate::font::Font;

/// An _enumeration_ of font subsets.
/// 
/// A font has one or more subsets.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
pub enum Subset {
    /// The _adlam_ font subset.
    Adlam,
    /// The _ethiopic_ font subset.
    Ethiopic,
    /// The _latin_ font subset.
    Latin,
    /// The _latin-ext_ font subset.
    LatinExt,
    /// The _menu_ font subset.
    Menu,
    /// The _sinhala_ font subset.
    Sinhala,
    /// The _vietnamese_ font subset.
    Vietnamese,
}

impl Subset {

    /// Returns families for the [`Subset`].
    pub fn families(&self) -> Vec<Family> {
        match self {
            Subset::Adlam => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ADLaMDisplay,
                ]
            }
            Subset::Ethiopic => {
                vec![
                    #[cfg(feature = "static")]
                    Family::AbyssinicaSIL,
                ]
            }
            Subset::Latin => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ABeeZee,
                    #[cfg(feature = "static")]
                    Family::ADLaMDisplay,
                    #[cfg(any(feature = "variable", feature = "static"))]
                    Family::AROneSans,
                    #[cfg(feature = "static")]
                    Family::Abel,
                    #[cfg(feature = "static")]
                    Family::AbhayaLibre,
                    #[cfg(feature = "static")]
                    Family::Aboreto,
                    #[cfg(feature = "static")]
                    Family::AbrilFatface,
                    #[cfg(feature = "static")]
                    Family::AbyssinicaSIL,
                    #[cfg(feature = "static")]
                    Family::Aclonica,
                    #[cfg(feature = "static")]
                    Family::Acme,
                    #[cfg(feature = "static")]
                    Family::Actor,
                ]
            }
            Subset::LatinExt => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ABeeZee,
                    #[cfg(feature = "static")]
                    Family::ADLaMDisplay,
                    #[cfg(any(feature = "variable", feature = "static"))]
                    Family::AROneSans,
                    #[cfg(feature = "static")]
                    Family::AbhayaLibre,
                    #[cfg(feature = "static")]
                    Family::Aboreto,
                    #[cfg(feature = "static")]
                    Family::AbrilFatface,
                    #[cfg(feature = "static")]
                    Family::AbyssinicaSIL,
                ]
            }
            Subset::Menu => {
                vec![
                    #[cfg(feature = "static")]
                    Family::ABeeZee,
                    #[cfg(feature = "static")]
                    Family::ADLaMDisplay,
                    #[cfg(any(feature = "variable", feature = "static"))]
                    Family::AROneSans,
                    #[cfg(feature = "static")]
                    Family::Abel,
                    #[cfg(feature = "static")]
                    Family::AbhayaLibre,
                    #[cfg(feature = "static")]
                    Family::Aboreto,
                    #[cfg(feature = "static")]
                    Family::AbrilFatface,
                    #[cfg(feature = "static")]
                    Family::AbyssinicaSIL,
                    #[cfg(feature = "static")]
                    Family::Aclonica,
                    #[cfg(feature = "static")]
                    Family::Acme,
                    #[cfg(feature = "static")]
                    Family::Actor,
                ]
            }
            Subset::Sinhala => {
                vec![
                    #[cfg(feature = "static")]
                    Family::AbhayaLibre,
                ]
            }
            Subset::Vietnamese => {
                vec![
                    #[cfg(any(feature = "variable", feature = "static"))]
                    Family::AROneSans,
                ]
            }
        }
    }

    /// Returns fonts for the [`Subset`].
    pub fn fonts(&self) -> Vec<Font> {
        match self {
            Subset::Adlam => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
                ]
            }
            Subset::Ethiopic => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                ]
            }
            Subset::Latin => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ABeeZeeRegular,
                    #[cfg(feature = "static")]
                    Font::ABeeZeeItalic,
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
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
                    Font::AboretoRegular,
                    #[cfg(feature = "static")]
                    Font::AbrilFatfaceRegular,
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                    #[cfg(feature = "static")]
                    Font::AclonicaRegular,
                    #[cfg(feature = "static")]
                    Font::AcmeRegular,
                    #[cfg(feature = "static")]
                    Font::ActorRegular,
                ]
            }
            Subset::LatinExt => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ABeeZeeRegular,
                    #[cfg(feature = "static")]
                    Font::ABeeZeeItalic,
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
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
                    Font::AboretoRegular,
                    #[cfg(feature = "static")]
                    Font::AbrilFatfaceRegular,
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                ]
            }
            Subset::Menu => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ABeeZeeRegular,
                    #[cfg(feature = "static")]
                    Font::ABeeZeeItalic,
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
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
                    Font::AboretoRegular,
                    #[cfg(feature = "static")]
                    Font::AbrilFatfaceRegular,
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                    #[cfg(feature = "static")]
                    Font::AclonicaRegular,
                    #[cfg(feature = "static")]
                    Font::AcmeRegular,
                    #[cfg(feature = "static")]
                    Font::ActorRegular,
                ]
            }
            Subset::Sinhala => {
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
                ]
            }
            Subset::Vietnamese => {
                vec![
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
                ]
            }
        }
    }
}
