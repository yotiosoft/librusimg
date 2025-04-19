use std::path::{Path, PathBuf};
use std::fmt;
use image::DynamicImage;

pub mod backend;
pub use backend::*;

/// RusImg object.
/// This object contains an image object and its metadata.
pub struct RusImg {
    pub extension: Extension,
    pub data: Box<(dyn BackendTrait)>,
}

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
    InvalidTrimXY,
    ImageFormatCannotBeCompressed,
    UnsupportedFileExtension,
    UnsupportedFeature,
    ImageDataIsNone,
    FailedToGetDynamicImage,
    FailedToConvertExtension,
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
            RusimgError::InvalidTrimXY => write!(f, "Invalid trim XY"),
            RusimgError::ImageFormatCannotBeCompressed => write!(f, "this image format cannot be compressed"),
            RusimgError::UnsupportedFileExtension => write!(f, "Unsupported file extension"),
            RusimgError::UnsupportedFeature => write!(f, "Unsupported feature"),
            RusimgError::ImageDataIsNone => write!(f, "Image data is None"),
            RusimgError::FailedToGetDynamicImage => write!(f, "Failed to get dynamic image"),
            RusimgError::FailedToConvertExtension => write!(f, "Failed to convert extension"),
            RusimgError::ImageNotSpecified => write!(f, "Image not specified"),
            RusimgError::SourcePathMustBeSpecified => write!(f, "Source path must be specified"),
        }
    }
}


/// Rectangle object for rusimg.
/// This object is used for trimming an image.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Image size object.
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct ImgSize {
    pub width: usize,
    pub height: usize,
}
impl ImgSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
        }
    }
}

/// Save status object.
/// This object is used for tracking the status of saving an image.
/// It contains the output file path, the file size before saving, and the file size after saving.
/// If the image has compression, the file size after saving will be different from the file size before saving.
#[derive(Debug, Clone, PartialEq)]
pub struct SaveStatus {
    pub output_path: Option<PathBuf>,
    pub before_filesize: Option<u64>,
    pub after_filesize: Option<u64>,
}

/// Image extension object.
/// By default, Rusimg supports BMP, JPEG, PNG, and WebP.
/// If you want to use another format, you can use ExternalFormat like ``Extension::ExternalFormat("tiff".to_string())``.
#[derive(Debug, Clone, PartialEq)]
pub enum Extension {
    Empty,
    Bmp,
    Jpeg,
    Png,
    Webp,
    ExternalFormat(String),
}
impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Extension::Empty => write!(f, "empty"),
            Extension::Bmp => write!(f, "bmp"),
            Extension::Jpeg => write!(f, "jpeg"),
            Extension::Png => write!(f, "png"),
            Extension::Webp => write!(f, "webp"),
            Extension::ExternalFormat(s) => write!(f, "{}", s),
        }
    }
}

/// RusImg object implementation.
/// The RusImg object wraps BackendTrait functions.
impl RusImg {
    /// Open an image file.
    /// This function will open an image file and return a RusImg object.
    /// The image file will be opened based on the file extension.
    pub fn open(path: &Path) -> Result<Self, RusimgError> {
        backend::open_image(path)
    }

    /// Get image size.
    /// This uses the ``get_size()`` function from ``BackendTrait``.
    pub fn get_image_size(&self) -> Result<ImgSize, RusimgError> {
        let size = self.data.get_size()?;
        Ok(size)
    }

    /// Resize an image.
    /// It must be called after open_image().
    /// Set ratio to 100 to keep the original size.
    /// This uses the ``resize()`` function from ``BackendTrait``.
    pub fn resize(&mut self, ratio: u8) -> Result<ImgSize, RusimgError> {
        let size = self.data.resize(ratio)?;
        Ok(size)
    }

    /// Trim an image. Set the trim area with four u32 values: x, y, w, h.
    /// It must be called after open_image().
    /// The values will be assigned to a Rect object.
    /// This uses the ``trim()`` function from ``BackendTrait``.
    pub fn trim(&mut self, trim_x: u32, trim_y: u32, trim_w: u32, trim_h: u32) -> Result<ImgSize, RusimgError> {
        let size = self.data.trim(Rect{x: trim_x, y: trim_y, w: trim_w, h: trim_h})?;
        Ok(size)
    }
    /// Trim an image. Set the trim area with a rusimg::Rect object.
    /// It must be called after open_image().
    /// This uses the ``trim()`` function from ``BackendTrait``.
    pub fn trim_rect(&mut self, trim_area: Rect) -> Result<ImgSize, RusimgError> {
        let size = self.data.trim(trim_area)?;
        Ok(size)
    }

    /// Grayscale an image.
    /// It must be called after open_image().
    /// This uses the ``grayscale()`` function from ``BackendTrait``.
    pub fn grayscale(&mut self) -> Result<(), RusimgError> {
        self.data.grayscale();
        Ok(())
    }

    /// Compress an image.
    /// It must be called after open_image().
    /// Set quality to 100 to keep the original quality.
    /// This uses the ``compress()`` function from ``BackendTrait``.
    pub fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError> {
        self.data.compress(quality)?;
        Ok(())
    }

    /// Convert an image to another format.
    /// And replace the original image with the new one.
    /// It must be called after open_image().
    /// This uses the ``get_dynamic_image()`` function to get the DynamicImage object, ``get_metadata_src()`` to get the metadata, and ``compress()`` to compress the image.
    pub fn convert(&mut self, new_extension: &Extension) -> Result<(), RusimgError> {
        let dynamic_image = self.data.get_dynamic_image()?;
        let filepath = self.data.get_source_filepath();
        let metadata = self.data.get_metadata_src();

        let new_image: Box<(dyn BackendTrait)> = match new_extension {
            Extension::Empty => return Err(RusimgError::UnsupportedFileExtension),
            Extension::Bmp => {
                backend::convert_to_bmp_image(dynamic_image, filepath, metadata)?
            },
            Extension::Jpeg => {
                backend::convert_to_jpeg_image(dynamic_image, filepath, metadata)?
            },
            Extension::Png => {
                backend::convert_to_png_image(dynamic_image, filepath, metadata)?
            },
            Extension::Webp => {
                backend::convert_to_webp_image(dynamic_image, filepath, metadata)?
            },
            Extension::ExternalFormat(_) => return Err(RusimgError::UnsupportedFileExtension),
        };

        self.extension = new_extension.clone();
        self.data = new_image;

        Ok(())
    }

    /// Set a ``image::DynamicImage`` to an RusImg.
    /// After setting the image, the image object will be updated.
    /// This uses the ``set_dynamic_image()`` function from ``BackendTrait``.
    pub fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError> {
        self.data.set_dynamic_image(image)?;
        Ok(())
    }

    /// Get a ``image::DynamicImage`` from an RusImg.
    /// This uses the ``get_dynamic_image()`` function from ``BackendTrait``.
    pub fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError> {
        let dynamic_image = self.data.get_dynamic_image()?;
        Ok(dynamic_image)
    }

    /// Get file extension.
    /// This returns the file extension of the image.
    pub fn get_extension(&self) -> Extension {
        self.extension.clone()
    }

    /// Get input file path.
    /// This returns the file path of the image.
    pub fn get_input_filepath(&self) -> Result<PathBuf, RusimgError> {
        self.data.get_source_filepath().ok_or(RusimgError::SourcePathMustBeSpecified)
    }

    /// Save an image to a file.
    /// If path is None, the original file will be overwritten.
    /// This uses the ``get_destination_filepath()`` to get the destination file path, ``get_metadata_src()`` to get the source file size, and ``get_metadata_dest()`` to get the destination file size, and ``save()`` to save the image.
    pub fn save_image(&mut self, path: Option<&str>) -> Result<SaveStatus, RusimgError> {
        let path_buf = match path {
            Some(p) => Some(PathBuf::from(p)),
            None => None,
        };
        self.data.save(path_buf)?;

        let ret = SaveStatus {
            output_path: self.data.get_destination_filepath()?.clone().or(None),
            before_filesize: if let Some(m) = self.data.get_metadata_src() {
                Some(m.len())
            } else {
                None
            },
            after_filesize: if let Some(m) = self.data.get_metadata_dest() {
                Some(m.len())
            } else {
                None
            },
        };
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use image::{ImageBuffer, Rgb};

    // 画像を生成する関数
    fn generate_test_image(filename: &str, width: u32, height: u32) {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        for x in 0..width {
            for y in 0..height {
                let r = (x * 3) as u8;
                let g = (y * 5) as u8;
                let b = (x * y) as u8;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        let mut test_image = RusImg::open(Path::new(filename)).unwrap();
        test_image.data.set_dynamic_image(DynamicImage::ImageRgb8(img)).unwrap();
        let new_extension = Extension::Png;
        test_image.convert(&new_extension).unwrap();
        test_image.save_image(Some(filename)).unwrap();
    }

    #[test]
    fn test_open_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let result = RusImg::open(path);
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_image_size() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let size = img.get_image_size().unwrap();
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 100);
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_resize_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let size = img.resize(50).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_trim_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let size = img.trim(10, 10, 50, 50).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_trim_rect_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let rect = Rect { x: 10, y: 10, w: 50, h: 50 };
        let size = img.trim_rect(rect).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_grayscale_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.grayscale();
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_compress_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.compress(Some(80.0));
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_convert_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.convert(&Extension::Jpeg);
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_set_dynamic_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let dynamic_image = image::open(path).unwrap();
        let result = img.set_dynamic_image(dynamic_image);
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_dynamic_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.get_dynamic_image();
        assert!(result.is_ok());
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_extension() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let extension = img.get_extension();
        assert_eq!(extension, Extension::Png);
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_input_filepath() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let input_filepath = img.get_input_filepath().unwrap();
        assert_eq!(input_filepath, Path::new(filename));
        //std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_save_image() {
        let filename = "test_image.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.save_image(Some("test_image_saved.png"));
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
        //std::fs::remove_file("test_image_saved.png").unwrap();
    }
}
