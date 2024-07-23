use google_fonts::Font;
use serde::{Deserialize, Serialize};

fn main() {
    let doc1 = Doc {
        font: Font::LemonadaRegular,
        text: "Hello, Doc!".into(),
    };
    let json_str = serde_json::to_string(&doc1).unwrap();
    eprintln!("{}", json_str);
    let doc2: Doc = serde_json::from_str(&json_str).unwrap();
    assert_eq!(doc1, doc2)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Doc {
    pub font: Font,
    pub text: String,
}
