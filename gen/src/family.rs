
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
pub enum Family {
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) font family.
    ABeeZee = 0,
    /// The [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) font family.
    ADLaMDisplay = 1000,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) font family.
    AROneSans = 2000,
    /// The [Abel](https://fonts.google.com/specimen/Abel) font family.
    Abel = 3000,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) font family.
    AbhayaLibre = 4000,
    /// The [Aboreto](https://fonts.google.com/specimen/Aboreto) font family.
    Aboreto = 5000,
    /// The [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) font family.
    AbrilFatface = 6000,
    /// The [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) font family.
    AbyssinicaSIL = 7000,
    /// The [Aclonica](https://fonts.google.com/specimen/Aclonica) font family.
    Aclonica = 8000,
    /// The [Acme](https://fonts.google.com/specimen/Acme) font family.
    Acme = 9000,
    /// The [Actor](https://fonts.google.com/specimen/Actor) font family.
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
            0 => Some(Family::ABeeZee),
            1000 => Some(Family::ADLaMDisplay),
            2000 => Some(Family::AROneSans),
            3000 => Some(Family::Abel),
            4000 => Some(Family::AbhayaLibre),
            5000 => Some(Family::Aboreto),
            6000 => Some(Family::AbrilFatface),
            7000 => Some(Family::AbyssinicaSIL),
            8000 => Some(Family::Aclonica),
            9000 => Some(Family::Acme),
            10000 => Some(Family::Actor),
            _ => None,
        }
    }

    /// The name of the font [`Family`] with spaces.
    pub fn name(&self) -> String {
        match self {
            Family::ABeeZee => "ABeeZee".into(),
            Family::ADLaMDisplay => "ADLaM Display".into(),
            Family::AROneSans => "AR One Sans".into(),
            Family::Abel => "Abel".into(),
            Family::AbhayaLibre => "Abhaya Libre".into(),
            Family::Aboreto => "Aboreto".into(),
            Family::AbrilFatface => "Abril Fatface".into(),
            Family::AbyssinicaSIL => "Abyssinica SIL".into(),
            Family::Aclonica => "Aclonica".into(),
            Family::Acme => "Acme".into(),
            Family::Actor => "Actor".into(),
        }
    }

    /// Returns fonts within the [`Family`].
    pub fn fonts(&self) -> Vec<Font> {
        match self {
            Family::ABeeZee => {
                vec![
                    Font::ABeeZeeRegular,
                    Font::ABeeZeeItalic,
                ]
            }
            Family::ADLaMDisplay => {
                vec![
                    Font::ADLaMDisplayRegular,
                ]
            }
            Family::AROneSans => {
                vec![
                    Font::AROneSansRegular,
                    Font::AROneSansMedium,
                    Font::AROneSansSemiBold,
                    Font::AROneSansBold,
                    Font::AROneSansVariable,
                ]
            }
            Family::Abel => {
                vec![
                    Font::AbelRegular,
                ]
            }
            Family::AbhayaLibre => {
                vec![
                    Font::AbhayaLibreRegular,
                    Font::AbhayaLibreMedium,
                    Font::AbhayaLibreSemiBold,
                    Font::AbhayaLibreBold,
                    Font::AbhayaLibreExtraBold,
                ]
            }
            Family::Aboreto => {
                vec![
                    Font::AboretoRegular,
                ]
            }
            Family::AbrilFatface => {
                vec![
                    Font::AbrilFatfaceRegular,
                ]
            }
            Family::AbyssinicaSIL => {
                vec![
                    Font::AbyssinicaSILRegular,
                ]
            }
            Family::Aclonica => {
                vec![
                    Font::AclonicaRegular,
                ]
            }
            Family::Acme => {
                vec![
                    Font::AcmeRegular,
                ]
            }
            Family::Actor => {
                vec![
                    Font::ActorRegular,
                ]
            }
        }
    }
    /// Returns the font [`Category`].
    pub fn category(&self) -> Category {
        match self {
            Family::ABeeZee => Category::SansSerif,
            Family::ADLaMDisplay => Category::Display,
            Family::AROneSans => Category::SansSerif,
            Family::Abel => Category::SansSerif,
            Family::AbhayaLibre => Category::Serif,
            Family::Aboreto => Category::Display,
            Family::AbrilFatface => Category::Display,
            Family::AbyssinicaSIL => Category::Serif,
            Family::Aclonica => Category::SansSerif,
            Family::Acme => Category::SansSerif,
            Family::Actor => Category::SansSerif,
        }
    }
}
