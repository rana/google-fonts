
// pub mod category;
// pub mod error;
// pub mod family;
// pub mod font;
    
use crate::font::Font;
use crate::error::FontError;


/// Get font data for the [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn abeezee_regular() -> Result<Vec<u8>, FontError> {
    Font::ABeeZeeRegular.get_and_cache()
}

/// Get font data for the [ABeeZee](https://fonts.google.com/specimen/ABeeZee) _italic_ font.
///
/// Loaded from the network and cached to disk.
pub fn abeezee_italic() -> Result<Vec<u8>, FontError> {
    Font::ABeeZeeItalic.get_and_cache()
}

/// Get font data for the [ADLaM Display](https://fonts.google.com/specimen/ADLaM+Display) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn adlam_display_regular() -> Result<Vec<u8>, FontError> {
    Font::ADLaMDisplayRegular.get_and_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn ar_one_sans_regular() -> Result<Vec<u8>, FontError> {
    Font::AROneSansRegular.get_and_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _medium_ font.
///
/// Loaded from the network and cached to disk.
pub fn ar_one_sans_medium() -> Result<Vec<u8>, FontError> {
    Font::AROneSansMedium.get_and_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _semi bold_ font.
///
/// Loaded from the network and cached to disk.
pub fn ar_one_sans_semi_bold() -> Result<Vec<u8>, FontError> {
    Font::AROneSansSemiBold.get_and_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _bold_ font.
///
/// Loaded from the network and cached to disk.
pub fn ar_one_sans_bold() -> Result<Vec<u8>, FontError> {
    Font::AROneSansBold.get_and_cache()
}

/// Get font data for the [AR One Sans](https://fonts.google.com/specimen/AR+One+Sans) _variable_ font.
///
/// Loaded from the network and cached to disk.
pub fn ar_one_sans_variable() -> Result<Vec<u8>, FontError> {
    Font::AROneSansVariable.get_and_cache()
}

/// Get font data for the [Abel](https://fonts.google.com/specimen/Abel) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn abel_regular() -> Result<Vec<u8>, FontError> {
    Font::AbelRegular.get_and_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn abhaya_libre_regular() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreRegular.get_and_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _medium_ font.
///
/// Loaded from the network and cached to disk.
pub fn abhaya_libre_medium() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreMedium.get_and_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _semi bold_ font.
///
/// Loaded from the network and cached to disk.
pub fn abhaya_libre_semi_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreSemiBold.get_and_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _bold_ font.
///
/// Loaded from the network and cached to disk.
pub fn abhaya_libre_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreBold.get_and_cache()
}

/// Get font data for the [Abhaya Libre](https://fonts.google.com/specimen/Abhaya+Libre) _extra bold_ font.
///
/// Loaded from the network and cached to disk.
pub fn abhaya_libre_extra_bold() -> Result<Vec<u8>, FontError> {
    Font::AbhayaLibreExtraBold.get_and_cache()
}

/// Get font data for the [Aboreto](https://fonts.google.com/specimen/Aboreto) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn aboreto_regular() -> Result<Vec<u8>, FontError> {
    Font::AboretoRegular.get_and_cache()
}

/// Get font data for the [Abril Fatface](https://fonts.google.com/specimen/Abril+Fatface) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn abril_fatface_regular() -> Result<Vec<u8>, FontError> {
    Font::AbrilFatfaceRegular.get_and_cache()
}

/// Get font data for the [Abyssinica SIL](https://fonts.google.com/specimen/Abyssinica+SIL) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn abyssinica_sil_regular() -> Result<Vec<u8>, FontError> {
    Font::AbyssinicaSILRegular.get_and_cache()
}

/// Get font data for the [Aclonica](https://fonts.google.com/specimen/Aclonica) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn aclonica_regular() -> Result<Vec<u8>, FontError> {
    Font::AclonicaRegular.get_and_cache()
}

/// Get font data for the [Acme](https://fonts.google.com/specimen/Acme) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn acme_regular() -> Result<Vec<u8>, FontError> {
    Font::AcmeRegular.get_and_cache()
}

/// Get font data for the [Actor](https://fonts.google.com/specimen/Actor) _regular_ font.
///
/// Loaded from the network and cached to disk.
pub fn actor_regular() -> Result<Vec<u8>, FontError> {
    Font::ActorRegular.get_and_cache()
}

/// Fonts which use _variable_ font technology.
pub fn variable_fonts() -> Vec<Font> {
    vec![
        Font::AROneSansVariable,
    ]
}

/// Fonts which use _static_ font technology.
pub fn static_fonts() -> Vec<Font> {
    vec![
        Font::ABeeZeeRegular,
        Font::ABeeZeeItalic,
        Font::ADLaMDisplayRegular,
        Font::AROneSansRegular,
        Font::AROneSansMedium,
        Font::AROneSansSemiBold,
        Font::AROneSansBold,
        Font::AbelRegular,
        Font::AbhayaLibreRegular,
        Font::AbhayaLibreMedium,
        Font::AbhayaLibreSemiBold,
        Font::AbhayaLibreBold,
        Font::AbhayaLibreExtraBold,
        Font::AboretoRegular,
        Font::AbrilFatfaceRegular,
        Font::AbyssinicaSILRegular,
        Font::AclonicaRegular,
        Font::AcmeRegular,
        Font::ActorRegular,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttf_parser::Face;

    #[test]
    fn test_abeezee_regular() {
        let result = abeezee_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abeezee_italic() {
        let result = abeezee_italic();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_adlam_display_regular() {
        let result = adlam_display_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_ar_one_sans_regular() {
        let result = ar_one_sans_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_ar_one_sans_medium() {
        let result = ar_one_sans_medium();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_ar_one_sans_semi_bold() {
        let result = ar_one_sans_semi_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_ar_one_sans_bold() {
        let result = ar_one_sans_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_ar_one_sans_variable() {
        let result = ar_one_sans_variable();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abel_regular() {
        let result = abel_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abhaya_libre_regular() {
        let result = abhaya_libre_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abhaya_libre_medium() {
        let result = abhaya_libre_medium();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abhaya_libre_semi_bold() {
        let result = abhaya_libre_semi_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abhaya_libre_bold() {
        let result = abhaya_libre_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abhaya_libre_extra_bold() {
        let result = abhaya_libre_extra_bold();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_aboreto_regular() {
        let result = aboreto_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abril_fatface_regular() {
        let result = abril_fatface_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_abyssinica_sil_regular() {
        let result = abyssinica_sil_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_aclonica_regular() {
        let result = aclonica_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_acme_regular() {
        let result = acme_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_actor_regular() {
        let result = actor_regular();
        assert!(result.is_ok());
        let font_data = result.unwrap();
        let result2 = Face::parse(&font_data, 0);
        assert!(result2.is_ok());
    }
}
