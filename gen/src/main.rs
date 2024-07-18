mod gen;
use anyhow::Result;
// mod category;
// mod error;
// mod family;
// pub mod font;
// mod lib2;

pub fn main() -> Result<()> {
    gen::build("src/", true)
}
