mod empty;
#[cfg(feature="bmp")]
mod bmp;
#[cfg(feature="jpeg")]
mod jpeg;
#[cfg(feature="png")]
mod png;
#[cfg(feature="webp")]
mod webp;

use std::fs::Metadata;
use std::io::Read;
use std::path::{Path, PathBuf};
use image::DynamicImage;

use super::{RusImg, Extension, RusimgError, ImgSize, Rect};

/// BackendTrait is a trait for RusImg objects.
/// This trait is used for image operations.
/// Implement this trait for each image format.
pub trait BackendTrait {
    /// Import an image from a DynamicImage object.
    /// 
    /// args:
    /// - image: DynamicImage object
    /// - source_path: PathBuf object
    /// - source_metadata: Metadata object
    /// 
    /// returns:
    /// - Self object
    fn import(image: Option<DynamicImage>, source_path: Option<PathBuf>, source_metadata: Option<Metadata>) -> Result<Self, RusimgError> where Self: Sized;
    /// Open an image from a image buffer.
    /// The ``path`` parameter is the file path of the image, but it is used for copying the file path to the object.
    /// This returns a RusImg object.
    /// 
    /// args:
    /// - path: file path of the image
    /// - image_buf: image buffer
    /// - metadata: Metadata object
    /// 
    /// returns:
    /// - Self object
    fn open(path: Option<PathBuf>, image_buf: Option<Vec<u8>>, metadata: Option<Metadata>) -> Result<Self, RusimgError> where Self: Sized;

    /// Save the image to a file to the ``path``.
    /// If the ``path`` is None, the image will be saved to the original file with the new extension.
    /// 
    /// args:
    /// - path: file path for saving the image
    /// 
    /// returns:
    /// - Result object
    fn save(&mut self, path: Option<PathBuf>) -> Result<(), RusimgError>;
    /// Compress the image with the quality parameter.
    /// The quality parameter is a float value between 0.0 and 100.0.
    /// 
    /// args:
    /// - quality: quality parameter
    /// 
    /// returns:
    /// - Result object
    fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError>;
    /// Resize the image with the resize_ratio parameter.
    /// The resize_ratio parameter is a u8 value between 1 and 100.
    /// 
    /// args:
    /// - resize_ratio: resize ratio parameter
    /// 
    /// returns:
    /// - ImgSize object
    fn resize(&mut self, resize_ratio: u8) -> Result<ImgSize, RusimgError>;
    /// Trim the image with the trim parameter.
    /// The trim parameter is a Rect object.
    /// 
    /// args:
    /// - trim: trim parameter (Rect object)
    /// 
    /// returns:
    /// - ImgSize object
    fn trim(&mut self, trim: Rect) -> Result<ImgSize, RusimgError>;
    /// Grayscale the image.
    fn grayscale(&mut self);
    /// Set a image::DynamicImage to the image object.
    /// After setting the image, the image object will be updated.
    /// 
    /// args:
    /// - image: DynamicImage object
    /// 
    /// returns:
    /// - Result object
    fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError>;
    /// Get a image::DynamicImage from the image object.
    /// 
    /// returns:
    /// - DynamicImage object
    fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError>;
    /// Get the source file path.
    /// 
    /// returns:
    /// - Result<PathBuf, RusimgError>
    fn get_source_filepath(&self) -> Option<PathBuf>;
    /// Get the destination file path.
    /// 
    /// returns:
    /// - Result<Option<PathBuf>, RusimgError>
    fn get_destination_filepath(&self) -> Result<Option<PathBuf>, RusimgError>;
    /// Get the source metadata.
    /// 
    /// returns:
    /// - Result<Metadata, RusimgError>
    fn get_metadata_src(&self) -> Option<Metadata>;
    /// Get the destination metadata.
    /// 
    /// returns:
    /// - Result<Option<Metadata>, RusimgError>
    fn get_metadata_dest(&self) -> Option<Metadata>;
    /// Get the image size.
    /// 
    /// returns:
    /// - Result<ImgSize, RusimgError>
    fn get_size(&self) -> Result<ImgSize, RusimgError>;

    /// Get a file path for saving an image.
    /// If the destination_filepath is None, the image will be saved to the source file path with the new extension.
    /// 
    /// args:
    /// - source_filepath: source file path
    /// - destination_filepath: destination file path
    /// - new_extension: new extension
    /// 
    /// returns:
    /// - PathBuf object
    fn get_save_filepath(&self, source_filepath: &Option<PathBuf>, destination_filepath: Option<PathBuf>, new_extension: &String) -> Result<PathBuf, RusimgError> {
        if let Some(path) = destination_filepath {
            if Path::new(&path).is_dir() {
                let source_filepath = match source_filepath {
                    Some(path) => path,
                    None => return Err(RusimgError::SourcePathMustBeSpecified),
                };
                let filename = match Path::new(&source_filepath).file_name() {
                    Some(filename) => filename,
                    None => return Err(RusimgError::FailedToGetFilename(source_filepath.clone())),
                };
                Ok(Path::new(&path).join(filename).with_extension(new_extension))
            }
            else {
                Ok(path)
            }
        }
        else {
            let source_filepath = match source_filepath {
                Some(path) => path,
                None => return Err(RusimgError::SourcePathMustBeSpecified),
            };
            Ok(Path::new(&source_filepath).with_extension(new_extension))
        }
    }
}

// Get image format from image buffer.
fn guess_image_format(image_buf: &[u8]) -> Result<image::ImageFormat, RusimgError> {
    let format = image::guess_format(image_buf).map_err(|e| RusimgError::FailedToOpenImage(e.to_string()))?;
    Ok(format)
}

fn new_empty_image(path: PathBuf) -> Result<RusImg, RusimgError> {
    let empty = empty::EmptyImage::import(None, Some(path), None)?;
    let data = Box::new(empty);
    Ok(RusImg { extension: Extension::Empty, data: data })
}
/// Open a bmp image file and make a RusImg object.
/// If the bmp feature is enabled, it will open a BMP image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="bmp")]
fn open_bmp_image(path: &Path, buf: Vec<u8>, metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    let image = bmp::BmpImage::open(Some(path.to_path_buf()), Some(buf), Some(metadata_input))?;
    let data = Box::new(image);
    Ok(RusImg { extension: Extension::Bmp, data: data })
}
#[cfg(not(feature="bmp"))]
fn open_bmp_image(_path: &Path, _buf: Vec<u8>, _metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Open a jpeg image file and make a RusImg object.
/// If the jpeg feature is enabled, it will open a JPEG image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="jpeg")]
fn open_jpeg_image(path: &Path, buf: Vec<u8>, metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    let image = jpeg::JpegImage::open(Some(path.to_path_buf()), Some(buf), Some(metadata_input))?;
    let data = Box::new(image);
    Ok(RusImg { extension: Extension::Jpeg, data: data })
}
#[cfg(not(feature="jpeg"))]
fn open_jpeg_image(_path: &Path, _buf: Vec<u8>, _metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Open a png image file and make a RusImg object.
/// If the png feature is enabled, it will open a PNG image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="png")]
fn open_png_image(path: &Path, buf: Vec<u8>, metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    let image = png::PngImage::open(Some(path.to_path_buf()), Some(buf), Some(metadata_input))?;
    let data = Box::new(image);
    Ok(RusImg { extension: Extension::Png, data: data })
}
#[cfg(not(feature="png"))]
fn open_png_image(_path: &Path, _buf: Vec<u8>, _metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Open a webp image file and make a RusImg object.
/// If the webp feature is enabled, it will open a WebP image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="webp")]
fn open_webp_image(path: &Path, buf: Vec<u8>, metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    let image = webp::WebpImage::open(Some(path.to_path_buf()), Some(buf), Some(metadata_input))?;
    let data = Box::new(image);
    Ok(RusImg { extension: Extension::Webp, data: data })
}
#[cfg(not(feature="webp"))]
fn open_webp_image(_path: &Path, _buf: Vec<u8>, _metadata_input: Metadata) -> Result<RusImg, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}

/// Open an image file and return a RusImg object.
pub fn open_image(path: &Path) -> Result<RusImg, RusimgError> {
    // If the file does not exist, make an empty image.
    if !path.exists() {
        return new_empty_image(path.to_path_buf());
    }

    let mut raw_data = std::fs::File::open(&path.to_path_buf()).map_err(|e| RusimgError::FailedToOpenFile(e.to_string()))?;
    let mut buf = Vec::new();
    raw_data.read_to_end(&mut buf).map_err(|e| RusimgError::FailedToReadFile(e.to_string()))?;
    let metadata_input = raw_data.metadata().map_err(|e| RusimgError::FailedToGetMetadata(e.to_string()))?;

    match guess_image_format(&buf)? {
        image::ImageFormat::Bmp => {
            open_bmp_image(path, buf, metadata_input)
        },
        image::ImageFormat::Jpeg => {
            open_jpeg_image(path, buf, metadata_input)
        },
        image::ImageFormat::Png => {
            open_png_image(path, buf, metadata_input)
        },
        image::ImageFormat::WebP => {
            open_webp_image(path, buf, metadata_input)
        },
        _ => Err(RusimgError::UnsupportedFileExtension),
    }
}

/// Open but not read because the file is not exist.
pub fn new_image(path: &Path) -> Result<RusImg, RusimgError> {
    new_empty_image(path.to_path_buf())
}

// Converter interfaces.
/// Convert a DynamicImage object to a BMP image object.
/// If the bmp feature is enabled, it will convert the DynamicImage to a BMP image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="bmp")]
pub fn convert_to_bmp_image(dynamic_image: DynamicImage, filepath: Option<PathBuf>, metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    let bmp = bmp::BmpImage::import(Some(dynamic_image), filepath, metadata)?;
    Ok(Box::new(bmp))
}
#[cfg(not(feature="bmp"))]
pub fn convert_to_bmp_image(_dynamic_image: DynamicImage, _filepath: Option<PathBuf>, _metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Convert a DynamicImage object to a JPEG image object.
/// If the jpeg feature is enabled, it will convert the DynamicImage to a JPEG image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="jpeg")]
pub fn convert_to_jpeg_image(dynamic_image: DynamicImage, filepath: Option<PathBuf>, metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    let jpeg = jpeg::JpegImage::import(Some(dynamic_image), filepath, metadata)?;
    Ok(Box::new(jpeg))
}
#[cfg(not(feature="jpeg"))]
pub fn convert_to_jpeg_image(_dynamic_image: DynamicImage, _filepath: Option<PathBuf>, _metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Convert a DynamicImage object to a PNG image object.
/// If the png feature is enabled, it will convert the DynamicImage to a PNG image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="png")]
pub fn convert_to_png_image(dynamic_image: DynamicImage, filepath: Option<PathBuf>, metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    let png = png::PngImage::import(Some(dynamic_image), filepath, metadata)?;
    Ok(Box::new(png))
}
#[cfg(not(feature="png"))]
pub fn convert_to_png_image(_dynamic_image: DynamicImage, _filepath: Option<PathBuf>, _metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
/// Convert a DynamicImage object to a WebP image object.
/// If the webp feature is enabled, it will convert the DynamicImage to a WebP image.
/// If not, it will return an UnsupportedFileExtension error.
#[cfg(feature="webp")]
pub fn convert_to_webp_image(dynamic_image: DynamicImage, filepath: Option<PathBuf>, metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    let webp = webp::WebpImage::import(Some(dynamic_image), filepath, metadata)?;
    Ok(Box::new(webp))
}
#[cfg(not(feature="webp"))]
pub fn convert_to_webp_image(_dynamic_image: DynamicImage, _filepath: Option<PathBuf>, _metadata: Option<Metadata>) -> Result<Box<(dyn BackendTrait)>, RusimgError> {
    Err(RusimgError::UnsupportedFileExtension)
}
