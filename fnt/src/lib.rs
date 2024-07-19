
pub mod category;
pub mod error;
pub mod family;
pub mod font;
pub mod subset;

use crate::family::Family;
use crate::font::Font;
use crate::error::FontError;

/// Get font data for the [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Anja Meiners_.
///
/// ![ABeeZee Regular](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeRegular.webp)
#[cfg(feature = "static")]
pub fn abeezee_regular() -> Result<Vec<u8>, FontError> {
    Font::ABeeZeeRegular.get_with_cache()
}

/// Get font data for the [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _italic_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Anja Meiners_.
///
/// ![ABeeZee Italic](https://rana.github.io/google-fonts/doc/imgs/ABeeZeeItalic.webp)
#[cfg(feature = "static")]
pub fn abeezee_italic() -> Result<Vec<u8>, FontError> {
    Font::ABeeZeeItalic.get_with_cache()
}

/// Get font data for the [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mark Jamra_, _Neil Patel_, and _Andrew Footit_.
///
/// ![ADLaMDisplay Regular](https://rana.github.io/google-fonts/doc/imgs/ADLaMDisplayRegular.webp)
#[cfg(feature = "static")]
pub fn adlam_display_regular() -> Result<Vec<u8>, FontError> {
    Font::ADLaMDisplayRegular.get_with_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Niteesh Yadav_.
///
/// ![AROneSans Regular](https://rana.github.io/google-fonts/doc/imgs/AROneSansRegular.webp)
#[cfg(feature = "static")]
pub fn ar_one_sans_regular() -> Result<Vec<u8>, FontError> {
    Font::AROneSansRegular.get_with_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _medium_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Niteesh Yadav_.
///
/// ![AROneSans Medium](https://rana.github.io/google-fonts/doc/imgs/AROneSansMedium.webp)
#[cfg(feature = "static")]
pub fn ar_one_sans_medium() -> Result<Vec<u8>, FontError> {
    Font::AROneSansMedium.get_with_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _semi bold_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Niteesh Yadav_.
///
/// ![AROneSans SemiBold](https://rana.github.io/google-fonts/doc/imgs/AROneSansSemiBold.webp)
#[cfg(feature = "static")]
pub fn ar_one_sans_semi_bold() -> Result<Vec<u8>, FontError> {
    Font::AROneSansSemiBold.get_with_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _bold_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Niteesh Yadav_.
///
/// ![AROneSans Bold](https://rana.github.io/google-fonts/doc/imgs/AROneSansBold.webp)
#[cfg(feature = "static")]
pub fn ar_one_sans_bold() -> Result<Vec<u8>, FontError> {
    Font::AROneSansBold.get_with_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _variable_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Niteesh Yadav_.
///
/// ![AROneSans Variable](https://rana.github.io/google-fonts/doc/imgs/AROneSansVariable.webp)
#[cfg(feature = "variable")]
pub fn ar_one_sans_variable() -> Result<Vec<u8>, FontError> {
    Font::AROneSansVariable.get_with_cache()
}

/// Get font data for the [Abel](https://fonts.google.com/specimen/Abel) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _MADType_.
///
/// ![Abel Regular](https://rana.github.io/google-fonts/doc/imgs/AbelRegular.webp)
#[cfg(feature = "static")]
pub fn abel_regular() -> Result<Vec<u8>, FontError> {
    Font::AbelRegular.get_with_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mooniak_.
///
/// ![AbhayaLibre Regular](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreRegular.webp)
#[cfg(feature = "static")]
pub fn abhaya_libre_regular() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreRegular.get_with_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _medium_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mooniak_.
///
/// ![AbhayaLibre Medium](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreMedium.webp)
#[cfg(feature = "static")]
pub fn abhaya_libre_medium() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreMedium.get_with_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _semi bold_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mooniak_.
///
/// ![AbhayaLibre SemiBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreSemiBold.webp)
#[cfg(feature = "static")]
pub fn abhaya_libre_semi_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreSemiBold.get_with_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _bold_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mooniak_.
///
/// ![AbhayaLibre Bold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreBold.webp)
#[cfg(feature = "static")]
pub fn abhaya_libre_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreBold.get_with_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _extra bold_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Mooniak_.
///
/// ![AbhayaLibre ExtraBold](https://rana.github.io/google-fonts/doc/imgs/AbhayaLibreExtraBold.webp)
#[cfg(feature = "static")]
pub fn abhaya_libre_extra_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreExtraBold.get_with_cache()
}

/// Get font data for the [Aboreto](https://fonts.google.com/specimen/Aboreto) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Dominik Jáger_.
///
/// ![Aboreto Regular](https://rana.github.io/google-fonts/doc/imgs/AboretoRegular.webp)
#[cfg(feature = "static")]
pub fn aboreto_regular() -> Result<Vec<u8>, FontError> {
    Font::AboretoRegular.get_with_cache()
}

/// Get font data for the [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _TypeTogether_.
///
/// ![AbrilFatface Regular](https://rana.github.io/google-fonts/doc/imgs/AbrilFatfaceRegular.webp)
#[cfg(feature = "static")]
pub fn abril_fatface_regular() -> Result<Vec<u8>, FontError> {
    Font::AbrilFatfaceRegular.get_with_cache()
}

/// Get font data for the [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _SIL International_.
///
/// ![AbyssinicaSIL Regular](https://rana.github.io/google-fonts/doc/imgs/AbyssinicaSILRegular.webp)
#[cfg(feature = "static")]
pub fn abyssinica_sil_regular() -> Result<Vec<u8>, FontError> {
    Font::AbyssinicaSILRegular.get_with_cache()
}

/// Get font data for the [Aclonica](https://fonts.google.com/specimen/Aclonica) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Astigmatic_.
///
/// ![Aclonica Regular](https://rana.github.io/google-fonts/doc/imgs/AclonicaRegular.webp)
#[cfg(feature = "static")]
pub fn aclonica_regular() -> Result<Vec<u8>, FontError> {
    Font::AclonicaRegular.get_with_cache()
}

/// Get font data for the [Acme](https://fonts.google.com/specimen/Acme) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Juan Pablo del Peral_ and _Huerta Tipográfica_.
///
/// ![Acme Regular](https://rana.github.io/google-fonts/doc/imgs/AcmeRegular.webp)
#[cfg(feature = "static")]
pub fn acme_regular() -> Result<Vec<u8>, FontError> {
    Font::AcmeRegular.get_with_cache()
}

/// Get font data for the [Actor](https://fonts.google.com/specimen/Actor) _regular_ font.
///
/// Loaded from the network and cached to disk.
///
/// Designed by _Thomas Junold_.
///
/// ![Actor Regular](https://rana.github.io/google-fonts/doc/imgs/ActorRegular.webp)
#[cfg(feature = "static")]
pub fn actor_regular() -> Result<Vec<u8>, FontError> {
    Font::ActorRegular.get_with_cache()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttf_parser::Face;

    #[test]
    #[cfg(feature = "static")]
    fn test_cast_family_font() {
        let fam = Family::ABeeZee;
        let fnt = Font::ABeeZeeRegular;
        assert_eq!(fam, fnt.family());
        assert_eq!(fnt, fam.font());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abeezee_regular() {
        let result = abeezee_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abeezee_italic() {
        let result = abeezee_italic();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_adlam_display_regular() {
        let result = adlam_display_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_ar_one_sans_regular() {
        let result = ar_one_sans_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_ar_one_sans_medium() {
        let result = ar_one_sans_medium();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_ar_one_sans_semi_bold() {
        let result = ar_one_sans_semi_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_ar_one_sans_bold() {
        let result = ar_one_sans_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "variable")]
    fn test_ar_one_sans_variable() {
        let result = ar_one_sans_variable();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abel_regular() {
        let result = abel_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abhaya_libre_regular() {
        let result = abhaya_libre_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abhaya_libre_medium() {
        let result = abhaya_libre_medium();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abhaya_libre_semi_bold() {
        let result = abhaya_libre_semi_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abhaya_libre_bold() {
        let result = abhaya_libre_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abhaya_libre_extra_bold() {
        let result = abhaya_libre_extra_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_aboreto_regular() {
        let result = aboreto_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abril_fatface_regular() {
        let result = abril_fatface_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_abyssinica_sil_regular() {
        let result = abyssinica_sil_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_aclonica_regular() {
        let result = aclonica_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_acme_regular() {
        let result = acme_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_actor_regular() {
        let result = actor_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }
}
