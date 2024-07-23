use google_fonts::{Category, Family, Font, Subset};
use strum::IntoEnumIterator;

const CNT: usize = 3;

fn main() {
    // Enumerate categories.
    for category in Category::iter().take(CNT) {
        eprintln!("Category:{}", category);
        // Enumerate category families.
        eprintln!(" Families");
        for family in category.families().iter().take(CNT) {
            eprintln!("  {}", family);
        }
        // Enumerate category fonts.
        eprintln!(" Fonts");
        for font in category.fonts().iter().take(CNT) {
            eprintln!("  {}", font);
        }
    }
    eprintln!("---");

    // Enumerate subsets.
    for subset in Subset::iter().take(CNT) {
        eprintln!("Subset:{}", subset);
        // Enumerate subset families.
        eprintln!(" Families");
        for family in subset.families().iter().take(CNT) {
            eprintln!("  {}", family);
        }
        // Enumerate subset fonts.
        eprintln!(" Fonts");
        for font in subset.fonts().iter().take(CNT) {
            eprintln!("  {}", font);
        }
    }
    eprintln!("---");

    // Enumerate family.
    for family in Family::iter().take(CNT) {
        eprintln!("Family:{}", family);

        // Print family category.
        eprintln!(" Category");
        eprintln!("  {}", family.category());

        // Print family's first font.
        eprintln!(" First Font");
        eprintln!("  {}", family.font());

        // Enumerate family fonts.
        eprintln!(" Fonts");
        for font in family.fonts().iter().take(CNT) {
            eprintln!("  {}", font);
        }

        // Enumerate family coverage.
        eprintln!(" Coverage");
        for coverage in family.coverage().iter().take(CNT) {
            eprintln!("  {:?}", coverage);
        }
    }
    eprintln!("---");

    // Enumerate font.
    for font in Font::iter().take(CNT) {
        eprintln!("Family:{}", font);

        // Print font category.
        eprintln!(" Category");
        eprintln!("  {}", font.category());

        // Print font family
        eprintln!(" Family");
        eprintln!("  {}", font.family());

        // Print is_variable
        eprintln!(" IsVariable");
        eprintln!("  {}", font.is_variable());

        // Print is_static
        eprintln!(" IsStatic");
        eprintln!("  {}", font.is_static());
    }
}
