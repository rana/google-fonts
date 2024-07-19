
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// An error that can occur while using the `google-fonts` crate.
///
/// This enum represents various errors that can be encountered while
/// downloading, deserializing, caching, or otherwise handling fonts
/// from Google Fonts. Each variant corresponds to a specific type
/// of error that can occur during the process.
///
/// # Variants
///
/// - `Network`: Indicates an error that occurred while making a network request.
/// - `Deserialize`: Indicates an error that occurred while deserializing JSON data.
/// - `CacheDir`: Indicates an error that occurred while interacting with the cache directory.
/// - `CacheFile`: Indicates an error that occurred while interacting with a cache file.
#[derive(Debug)]
pub enum FontError {
    /// An error that occurred while making a network request.
    ///
    /// This variant wraps a `reqwest::Error`, which provides more details
    /// about the specific network-related error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::Network(e) = error {
    ///     println!("Network error: {}", e);
    /// }
    /// ```
    Network(reqwest::Error),

    /// An error that occurred while deserializing JSON data.
    ///
    /// This variant wraps a `serde_json::Error`, which provides more details
    /// about the specific deserialization error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::Deserialize(e) = error {
    ///     println!("Deserialization error: {}", e);
    /// }
    /// ```
    Deserialize(serde_json::Error),

    /// An error that occurred while interacting with the cache directory.
    ///
    /// This variant wraps a `StringError`, which provides more details
    /// about the specific error related to the cache directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::CacheDir(e) = error {
    ///     println!("Cache directory error: {}", e);
    /// }
    /// ```
    CacheDir(StringError),

    /// An error that occurred while interacting with a cache file.
    ///
    /// This variant wraps a `std::io::Error`, which provides more details
    /// about the specific I/O error that occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// use google_fonts::FontError;
    /// if let FontError::CacheFile(e) = error {
    ///     println!("Cache file error: {}", e);
    /// }
    /// ```
    CacheFile(std::io::Error),
}

impl Display for FontError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FontError::Network(e) => write!(f, "font network error: {}", e),
            FontError::Deserialize(e) => write!(f, "deserialization error: {}", e),
            FontError::CacheDir(e) => write!(f, "font cache directory error: {}", e),
            FontError::CacheFile(e) => write!(f, "font cache file error: {}", e),
        }
    }
}

impl Error for FontError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FontError::Network(e) => Some(e),
            FontError::Deserialize(e) => Some(e),
            FontError::CacheDir(e) => Some(e),
            FontError::CacheFile(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct StringError {
    msg: String,
}
impl StringError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
impl Display for StringError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}
impl Error for StringError {}

