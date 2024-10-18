# google-fonts

Rust library for downloading and caching Google Fonts
  - Provides access to `11,689` fonts and `1,708` font families
  - Supports both variable and static font technologies
  - Offers flexible API for font retrieval with caching options
  - Includes configurable features for optimizing build time and crate size
  - Similar functionality to [Android Studio's Downloadable Fonts](https://developer.android.com/develop/ui/views/text-and-emoji/downloadable-fonts) and [Google's Web Fonts API](https://developers.google.com/fonts/docs/developer_api)

# Example

Download font data with a few approaches.

```rust
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
```

# Crate features

`full`, `variable`, and `static` crate features are available.
* `variable` enables only fonts with [variable font technology](https://fonts.google.com/knowledge/using_variable_fonts_on_the_web).
* `static` enables only fonts with _static font technology_.
* `full` enables both `variable` and `static` features.

`variable` is the default feature.

Variable font technology is newer, more flexible, and provides style variations in one or two files. Static font technology uses more font files to accomplish the same thing. A majority of the fonts are in the `static` feature. 

Prefer the `variable` feature when possible.

Enable `variable` to significantly improve build time, crate size, and rust-analyzer performance.

# Doc comment font images

View font images from docs.

![tooltip](imgs/tooltip.png)
