use std::fmt;

/// Image extension object.
/// By default, Rusimg supports BMP, JPEG, PNG, and WebP.
/// If you want to use another format, you can use ExternalFormat like ``Extension::ExternalFormat("tiff".to_string())``.
#[derive(Debug, Clone, PartialEq)]
pub enum Extension {
    Bmp,
    Jpg,
    Jpeg,
    Png,
    Webp,
    ExternalFormat(String),
}
impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Extension::Bmp => write!(f, "bmp"),
            Extension::Jpg => write!(f, "jpg"),
            Extension::Jpeg => write!(f, "jpeg"),
            Extension::Png => write!(f, "png"),
            Extension::Webp => write!(f, "webp"),
            Extension::ExternalFormat(s) => write!(f, "{}", s),
        }
    }
}
