use google_fonts::lemonada_variable;
use google_fonts::Font::NotoSansRegular;
use google_fonts::Font::RobotoRegular;
use ttf_parser::Face;

fn main() {
    // Get and cache font data with a named function.
    let font_data = lemonada_variable().unwrap();
    let face = Face::parse(&font_data, 0).unwrap();
    eprintln!("Font data: {:?}", face);

    // Get and cache font data with an enum variant function.
    let font_data = NotoSansRegular.get_with_cache().unwrap();
    let face = Face::parse(&font_data, 0).unwrap();
    eprintln!("Font data: {:?}", face);

    // Get font data without caching by using an enum variant function.
    let font_data = RobotoRegular.get().unwrap();
    let face = Face::parse(&font_data, 0).unwrap();
    eprintln!("Font data: {:?}", face);
}
