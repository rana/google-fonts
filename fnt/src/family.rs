
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, AsRefStr};
use crate::font::Font;
use crate::category::Category;

/// The _family id_ increment.
/// 
/// The Roboto Serif font family has 721 fonts.
pub const ID_INCREMENT: u32 = 1000;

/// An _enumeration_ of [Google font](https://fonts.google.com) families.
/// 
/// A font family may have one or more fonts with different styles and sizes.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumCount, EnumIter, EnumString, AsRefStr)]
#[repr(u32)] // for `mem::transmute`
pub enum Family {
    /// The [ABeeZee](https://fonts.google.com/specimen/ABeeZee) font family.
    ///
    /// Designed by _Anja Meiners_.
    ///
    /// ![ABeeZee Regular](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeRegular.webp)
    ///
    /// ![ABeeZee Italic](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeItalic.webp)
    #[cfg(feature = "static")]
    ABeeZee = 0,
    /// The [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) font family.
    ///
    /// Designed by _Mark Jamra_, _Neil Patel_, and _Andrew Footit_.
    ///
    /// ![ADLaMDisplay Regular](https://rana.github.io/google-fonts/doc/imgs/ADLaMDisplayRegular.webp)
    #[cfg(feature = "static")]
    ADLaMDisplay = 1000,
    /// The [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) font family.
    ///
    /// Designed by _Niteesh Yadav_.
    ///
    /// ![AROneSans Regular](https://rana.github.io/google-fonts/doc/imgs/AROneSansRegular.webp)
    ///
    /// ![AROneSans Medium](https://rana.github.io/google-fonts/doc/imgs/AROneSansMedium.webp)
    ///
    /// ![AROneSans SemiBold](https://rana.github.io/google-fonts/doc/imgs/AROneSansSemiBold.webp)
    ///
    /// ![AROneSans Bold](https://rana.github.io/google-fonts/doc/imgs/AROneSansBold.webp)
    ///
    /// ![AROneSans Variable](https://rana.github.io/google-fonts/doc/imgs/AROneSansVariable.webp)
    #[cfg(any(feature = "variable", feature = "static"))]
    AROneSans = 2000,
    /// The [Abel](https://fonts.google.com/specimen/Abel) font family.
    ///
    /// Designed by _MADType_.
    ///
    /// ![Abel Regular](https://rana.github.io/google-fonts/doc/imgs/AbelRegular.webp)
    #[cfg(feature = "static")]
    Abel = 3000,
    /// The [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) font family.
    ///
    /// Designed by _Mooniak_.
    ///
    /// ![AbhayaLibre Regular](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreRegular.webp)
    ///
    /// ![AbhayaLibre Medium](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreMedium.webp)
    ///
    /// ![AbhayaLibre SemiBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreSemiBold.webp)
    ///
    /// ![AbhayaLibre Bold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreBold.webp)
    ///
    /// ![AbhayaLibre ExtraBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreExtraBold.webp)
    #[cfg(feature = "static")]
    AbhayaLibre = 4000,
    /// The [Aboreto](https://fonts.google.com/specimen/Aboreto) font family.
    ///
    /// Designed by _Dominik Jáger_.
    ///
    /// ![Aboreto Regular](https://rana.github.io/google-fonts/doc/imgs/AboretoRegular.webp)
    #[cfg(feature = "static")]
    Aboreto = 5000,
    /// The [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) font family.
    ///
    /// Designed by _TypeTogether_.
    ///
    /// ![AbrilFatface Regular](https://rana.github.io/google-fonts/doc/imgs/AbrilFatfaceRegular.webp)
    #[cfg(feature = "static")]
    AbrilFatface = 6000,
    /// The [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) font family.
    ///
    /// Designed by _SIL International_.
    ///
    /// ![AbyssinicaSIL Regular](https://rana.github.io/google-fonts/doc/imgs/AbyssinicaSILRegular.webp)
    #[cfg(feature = "static")]
    AbyssinicaSIL = 7000,
    /// The [Aclonica](https://fonts.google.com/specimen/Aclonica) font family.
    ///
    /// Designed by _Astigmatic_.
    ///
    /// ![Aclonica Regular](https://rana.github.io/google-fonts/doc/imgs/AclonicaRegular.webp)
    #[cfg(feature = "static")]
    Aclonica = 8000,
    /// The [Acme](https://fonts.google.com/specimen/Acme) font family.
    ///
    /// Designed by _Juan Pablo del Peral_ and _Huerta Tipográfica_.
    ///
    /// ![Acme Regular](https://rana.github.io/google-fonts/doc/imgs/AcmeRegular.webp)
    #[cfg(feature = "static")]
    Acme = 9000,
    /// The [Actor](https://fonts.google.com/specimen/Actor) font family.
    ///
    /// Designed by _Thomas Junold_.
    ///
    /// ![Actor Regular](https://rana.github.io/google-fonts/doc/imgs/ActorRegular.webp)
    #[cfg(feature = "static")]
    Actor = 10000,
}

impl Family {

    /// Returns the _id_ for the [`Family`].
    pub fn id(&self) -> u32 {
        *self as u32
    }

    /// Returns the first [`Font`].
    pub fn font(&self) -> Font {
       Font::from_id(self.id())
    }

    /// Transforms an _id_ into a [`Family`].
    pub(crate) fn from_id(id: u32) -> Self {
        unsafe { std::mem::transmute(id) }
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

    /// Returns fonts for the [`Family`].
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
