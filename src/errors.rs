use std::path::PathBuf;
use std::fmt;

/// Error type for Rusimg.
/// This error type is used in Rusimg functions.
/// Some error types have a string parameter to store the error message.
#[derive(Debug, Clone, PartialEq)]
pub enum RusimgError {
    FailedToOpenFile(String),
    FailedToReadFile(String),
    FailedToGetMetadata(String),
    FailedToOpenImage(String),
    FailedToSaveImage(String),
    FailedToCopyBinaryData(String),
    FailedToGetFilename(PathBuf),
    FailedToCreateFile(String),
    FailedToWriteFIle(String),
    FailedToDecodeWebp,
    FailedToEncodeWebp(String),
    FailedToCompressImage(Option<String>),
    FailedToConvertPathToString,
    InvalidCompressionLevel,
    InvalidTrimXY,
    ImageFormatCannotBeCompressed,
    UnsupportedFileExtension,
    UnsupportedFeature,
    ImageNotSpecified,
    SourcePathMustBeSpecified,
}
/// Implement Display trait for RusimgError.
impl fmt::Display for RusimgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RusimgError::FailedToOpenFile(s) => write!(f, "Failed to open file: \n\t{}", s),
            RusimgError::FailedToReadFile(s) => write!(f, "Failed to read file: \n\t{}", s),
            RusimgError::FailedToGetMetadata(s) => write!(f, "Failed to get metadata: \n\t{}", s),
            RusimgError::FailedToOpenImage(s) => write!(f, "Failed to open image: \n\t{}", s),
            RusimgError::FailedToSaveImage(s) => write!(f, "Failed to save image: \n\t{}", s),
            RusimgError::FailedToCopyBinaryData(s) => write!(f, "Failed to copy binary data to memory: \n\t{}", s),
            RusimgError::FailedToGetFilename(s) => write!(f, "Failed to get filename: \n\t{}", s.display()),
            RusimgError::FailedToCreateFile(s) => write!(f, "Failed to create file: \n\t{}", s),
            RusimgError::FailedToWriteFIle(s) => write!(f, "Failed to write file: \n\t{}", s),
            RusimgError::FailedToDecodeWebp => write!(f, "Failed to decode webp"),
            RusimgError::FailedToEncodeWebp(s) => write!(f, "Failed to encode webp: \n\t{}", s),
            RusimgError::FailedToCompressImage(s) => {
                if let Some(s) = s {
                    write!(f, "Failed to compress image: \n\t{}", s)
                }
                else {
                    write!(f, "Failed to compress image")
                }
            }
            RusimgError::FailedToConvertPathToString => write!(f, "Failed to convert path to string"),
            RusimgError::InvalidCompressionLevel => write!(f, "Invalid compression level"),
            RusimgError::InvalidTrimXY => write!(f, "Invalid trim XY"),
            RusimgError::ImageFormatCannotBeCompressed => write!(f, "this image format cannot be compressed"),
            RusimgError::UnsupportedFileExtension => write!(f, "Unsupported file extension"),
            RusimgError::UnsupportedFeature => write!(f, "Unsupported feature"),
            RusimgError::ImageNotSpecified => write!(f, "Image not specified"),
            RusimgError::SourcePathMustBeSpecified => write!(f, "Source path must be specified"),
        }
    }
}
