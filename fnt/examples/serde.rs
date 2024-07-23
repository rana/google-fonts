use google_fonts::Font;
use serde_json::json;

fn main() {
    let doc1 = Document {
        font: Font::LemonadaRegular,
        text: "Hello, Doc!".into(),
    };
    let json_str = json!(doc1);
    eprintln!("{}", json_str);
    let doc2: Document = serde_json::from_str(&json_str).unwrap();
    assert_eq!(doc1, doc2)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Document {
    pub font: Font,
    pub text: String,
}
