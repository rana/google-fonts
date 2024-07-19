use crate::category::Category;
use crate::font::Font;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString};

/// The _family id_ increment.
///
/// The Roboto Serif font family has 721 fonts.
pub const ID_INCREMENT: isize = 1000;

/// An _enumeration_ of [Google font](https://fonts.google.com) families.
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
pub enum Family {
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) font family.
    #[cfg(feature = "static")]
    ABeeZee = 0,
    /// The [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) font family.
    #[cfg(feature = "static")]
    ADLaMDisplay = 1000,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) font family.
    #[cfg(any(feature = "variable", feature = "static"))]
    AROneSans = 2000,
    /// The [Abel](https://fonts.google.com/specimen/Abel) font family.
    #[cfg(feature = "static")]
    Abel = 3000,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) font family.
    #[cfg(feature = "static")]
    AbhayaLibre = 4000,
    /// The [Aboreto](https://fonts.google.com/specimen/Aboreto) font family.
    #[cfg(feature = "static")]
    Aboreto = 5000,
    /// The [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) font family.
    #[cfg(feature = "static")]
    AbrilFatface = 6000,
    /// The [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) font family.
    #[cfg(feature = "static")]
    AbyssinicaSIL = 7000,
    /// The [Aclonica](https://fonts.google.com/specimen/Aclonica) font family.
    #[cfg(feature = "static")]
    Aclonica = 8000,
    /// The [Acme](https://fonts.google.com/specimen/Acme) font family.
    #[cfg(feature = "static")]
    Acme = 9000,
    /// The [Actor](https://fonts.google.com/specimen/Actor) font family.
    #[cfg(feature = "static")]
    Actor = 10000,
}

impl Family {
    /// Returns the _id_ for the [`Family`].
    pub fn id(&self) -> isize {
        *self as isize
    }

    /// Returns the default [`Font`].
    pub fn font(&self) -> Font {
        Font::from_id(self.id()).unwrap()
    }

    /// Converts an `isize` to a [`Family`].
    pub fn from_id(id: isize) -> Option<Self> {
        match id {
            #[cfg(feature = "static")]
            0 => Some(Family::ABeeZee),
            #[cfg(feature = "static")]
            1000 => Some(Family::ADLaMDisplay),
            #[cfg(any(feature = "variable", feature = "static"))]
            2000 => Some(Family::AROneSans),
            #[cfg(feature = "static")]
            3000 => Some(Family::Abel),
            #[cfg(feature = "static")]
            4000 => Some(Family::AbhayaLibre),
            #[cfg(feature = "static")]
            5000 => Some(Family::Aboreto),
            #[cfg(feature = "static")]
            6000 => Some(Family::AbrilFatface),
            #[cfg(feature = "static")]
            7000 => Some(Family::AbyssinicaSIL),
            #[cfg(feature = "static")]
            8000 => Some(Family::Aclonica),
            #[cfg(feature = "static")]
            9000 => Some(Family::Acme),
            #[cfg(feature = "static")]
            10000 => Some(Family::Actor),
            _ => None,
        }
    }

    /// The name of the font [`Family`] with spaces.
    pub fn name(&self) -> String {
        match self {
            #[cfg(feature = "static")]
            Family::ABeeZee => "ABeeZee".into(),
            #[cfg(feature = "static")]
            Family::ADLaMDisplay => "ADLaM Display".into(),
            #[cfg(any(feature = "variable", feature = "static"))]
            Family::AROneSans => "AR One Sans".into(),
            #[cfg(feature = "static")]
            Family::Abel => "Abel".into(),
            #[cfg(feature = "static")]
            Family::AbhayaLibre => "Abhaya Libre".into(),
            #[cfg(feature = "static")]
            Family::Aboreto => "Aboreto".into(),
            #[cfg(feature = "static")]
            Family::AbrilFatface => "Abril Fatface".into(),
            #[cfg(feature = "static")]
            Family::AbyssinicaSIL => "Abyssinica SIL".into(),
            #[cfg(feature = "static")]
            Family::Aclonica => "Aclonica".into(),
            #[cfg(feature = "static")]
            Family::Acme => "Acme".into(),
            #[cfg(feature = "static")]
            Family::Actor => "Actor".into(),
        }
    }

    /// Returns fonts within the [`Family`].
    pub fn fonts(&self) -> Vec<Font> {
        match self {
            #[cfg(feature = "static")]
            Family::ABeeZee => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ABeeZeeRegular,
                    #[cfg(feature = "static")]
                    Font::ABeeZeeItalic,
                ]
            }
            #[cfg(feature = "static")]
            Family::ADLaMDisplay => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ADLaMDisplayRegular,
                ]
            }
            #[cfg(any(feature = "variable", feature = "static"))]
            Family::AROneSans => {
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
            #[cfg(feature = "static")]
            Family::Abel => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AbelRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::AbhayaLibre => {
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
            #[cfg(feature = "static")]
            Family::Aboreto => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AboretoRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::AbrilFatface => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AbrilFatfaceRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::AbyssinicaSIL => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AbyssinicaSILRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::Aclonica => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AclonicaRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::Acme => {
                vec![
                    #[cfg(feature = "static")]
                    Font::AcmeRegular,
                ]
            }
            #[cfg(feature = "static")]
            Family::Actor => {
                vec![
                    #[cfg(feature = "static")]
                    Font::ActorRegular,
                ]
            }
        }
    }
    /// Returns the font [`Category`].
    pub fn category(&self) -> Category {
        match self {
            #[cfg(feature = "static")]
            Family::ABeeZee => Category::SansSerif,
            #[cfg(feature = "static")]
            Family::ADLaMDisplay => Category::Display,
            #[cfg(any(feature = "variable", feature = "static"))]
            Family::AROneSans => Category::SansSerif,
            #[cfg(feature = "static")]
            Family::Abel => Category::SansSerif,
            #[cfg(feature = "static")]
            Family::AbhayaLibre => Category::Serif,
            #[cfg(feature = "static")]
            Family::Aboreto => Category::Display,
            #[cfg(feature = "static")]
            Family::AbrilFatface => Category::Display,
            #[cfg(feature = "static")]
            Family::AbyssinicaSIL => Category::Serif,
            #[cfg(feature = "static")]
            Family::Aclonica => Category::SansSerif,
            #[cfg(feature = "static")]
            Family::Acme => Category::SansSerif,
            #[cfg(feature = "static")]
            Family::Actor => Category::SansSerif,
        }
    }
}
