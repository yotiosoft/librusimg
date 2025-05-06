use std::path::{Path, PathBuf};
use image::DynamicImage;

pub mod backend;
pub use backend::*;
pub mod structs;
pub use structs::*;
pub mod errors;
pub use errors::*;
pub mod extension;
pub use extension::*;

/// RusImg object.
/// This object contains an image object and its metadata.
pub struct RusImg {
    extension: Extension,
    data: Box<(dyn BackendTrait)>,
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

    /// New image object.
    /// This function will create a new image object based on the file extension.
    /// It will return a RusImg object.
    pub fn new(extension: &Extension, image: DynamicImage) -> Result<Self, RusimgError> {
        backend::new_image(extension, image)
    }

    /// Create a new RusImg object from an Extension and a BaclendTrait object.
    /// This function is for external formats.
    /// It will return a RusImg object.
    pub fn assemble(extension: &Extension, data: Box<(dyn BackendTrait)>) -> Result<Self, RusimgError> {
        let mut new_img = RusImg {
            extension: extension.clone(),
            data,
        };
        new_img.extension = extension.clone();
        Ok(new_img)
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
    pub fn resize(&mut self, ratio: f32) -> Result<ImgSize, RusimgError> {
        if ratio <= 0.0 {
            return Err(RusimgError::InvalidResizeRatio);
        }

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
        if quality.is_some() && (quality.unwrap() < 0.0 || quality.unwrap() > 100.0) {
            return Err(RusimgError::InvalidCompressionLevel);
        }

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
            Extension::Bmp => {
                backend::convert_to_bmp_image(dynamic_image, filepath, metadata)?
            },
            Extension::Jpeg => {
                backend::convert_to_jpeg_image(dynamic_image, filepath, metadata)?
            },
            Extension::Jpg => {
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

    /// Remove alpha channel from the image.
    /// Because JPEG does not support alpha channel, it's necessary to remove it before saving.
    pub fn remove_alpha_channel(&mut self) -> Result<(), RusimgError> {
        self.data.remove_alpha_channel()?;
        Ok(())
    }

    /// Get file extension.
    /// This returns the file extension of the image.
    pub fn get_extension(&self) -> Extension {
        self.extension.clone()
    }

    /// Get input file path.
    /// This returns the file path of the image.
    pub fn get_input_filepath(&self) -> Result<PathBuf, RusimgError> {
        self.data.get_source_filepath().ok_or(RusimgError::DestinationPathMustBeSpecified)
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
        let mut test_image = RusImg::new(&Extension::Png, DynamicImage::ImageRgb8(img.clone())).unwrap();
        test_image.save_image(Some(filename)).unwrap();
    }

    #[test]
    fn test_open_image() {
        let filename = "test_image1.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let result = RusImg::open(path);
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_image_size() {
        let filename = "test_image2.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let size = img.get_image_size().unwrap();
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 100);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_resize_image() {
        let filename = "test_image3.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let size = img.resize(50.0).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_trim_image() {
        let filename = "test_image4.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let size = img.trim(10, 10, 50, 50).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_trim_rect_image() {
        let filename = "test_image5.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let rect = Rect { x: 10, y: 10, w: 50, h: 50 };
        let size = img.trim_rect(rect).unwrap();
        assert_eq!(size.width, 50);
        assert_eq!(size.height, 50);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_grayscale_image() {
        let filename = "test_image6.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.grayscale();
        assert!(result.is_ok());
        // color check
        let dynamic_image = img.get_dynamic_image().unwrap();
        let img_data = dynamic_image.to_rgb8();
        for pixel in img_data.pixels() {
            assert_eq!(pixel[0], pixel[1]);
            assert_eq!(pixel[1], pixel[2]);
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_compress_image() {
        let filename = "test_image7.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.compress(Some(30.0));
        assert!(result.is_ok());
        // size check
        img.save_image(None).unwrap();
        let before_size = img.data.get_metadata_src().unwrap().len();
        let after_size = img.data.get_metadata_dest().unwrap().len();
        assert!(after_size < before_size);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_convert_image() {
        let filename = "test_image8.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.convert(&Extension::Webp);
        assert!(result.is_ok());
        // file types
        let rusimg_extensions = vec![Extension::Bmp, Extension::Jpeg, Extension::Jpg, Extension::Png, Extension::Webp];
        let image_extensions = vec![image::ImageFormat::Bmp, image::ImageFormat::Jpeg, image::ImageFormat::Jpeg, image::ImageFormat::Png, image::ImageFormat::WebP];
        for (ext, image_ext) in rusimg_extensions.iter().zip(image_extensions.iter()) {
            // Convert the image to the new format.
            let new_filename = filename.replace(".png", &format!("_output.{}", ext));
            let new_path = Path::new(&new_filename);
            let mut image_cloned = RusImg::open(&PathBuf::from(filename)).unwrap();
            image_cloned.convert(&ext).unwrap();
            image_cloned.save_image(new_path.to_str()).unwrap();
            // Check if the file extension is correct.
            let output_image_binary = std::fs::read(new_path).unwrap();
            let guessed_format = image::guess_format(&output_image_binary).unwrap();
            assert_eq!(guessed_format, *image_ext);
            // Clean up the test image file.
            std::fs::remove_file(new_path).unwrap();
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_set_dynamic_image() {
        let filename = "test_image9.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let dynamic_image = image::open(path).unwrap();
        let result = img.set_dynamic_image(dynamic_image);
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_dynamic_image() {
        let filename = "test_image10.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.get_dynamic_image();
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_remove_alpha_channel() {
        let filename = "test_image11.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.remove_alpha_channel();
        assert!(result.is_ok());
        // Check if the image has an alpha channel
        let dynamic_image = img.get_dynamic_image().unwrap();
        assert_eq!(dynamic_image.color(), image::ColorType::Rgb8);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_extension() {
        let filename = "test_image12.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let extension = img.get_extension();
        assert_eq!(extension, Extension::Png);
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_get_input_filepath() {
        let filename = "test_image13.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let input_filepath = img.get_input_filepath().unwrap();
        assert_eq!(input_filepath, Path::new(filename));
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_save_image() {
        let filename = "test_image14.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.save_image(Some("test_image_saved.png"));
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
        std::fs::remove_file("test_image_saved.png").unwrap();
    }

    #[test]
    fn test_err_failed_to_open_file() {
        let path = Path::new("non_existent_file.png");
        let result = RusImg::open(path);
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::FailedToOpenFile(_) = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
    }

    #[test]
    fn test_err_failed_to_open_image() {
        // Not supported image format
        let path = Path::new("test_image1.txt");
        // Create a dummy text file
        std::fs::write(path, "This is a test file.").unwrap();
        // Attempt to open the text file as an image
        let result = RusImg::open(path);
        // Remove the dummy text file
        std::fs::remove_file(path).unwrap();
        // Check if the error is as expected
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::FailedToOpenImage(_) = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
    }

    #[test]
    fn test_err_failed_to_save_image() {
        let filename = "test_image15.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.save_image(Some("test_image/invalid_path/test_image_saved.png"));
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::FailedToSaveImage(_) = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        // Clean up the test image file
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_err_invalid_compression_level() {
        let filename = "test_image16.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result1 = img.compress(Some(150.0));
        let result2 = img.compress(Some(-10.0));
        assert!(result1.is_err());
        assert!(result2.is_err());
        if let Err(e) = result1 {
            if let RusimgError::InvalidCompressionLevel = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        if let Err(e) = result2 {
            if let RusimgError::InvalidCompressionLevel = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_err_invalid_trim_xy() {
        let filename = "test_image17.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.trim(150, 150, 50, 50);
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::InvalidTrimXY = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_err_invalid_resize_ratio() {
        let filename = "test_image18.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.resize(0.0);
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::InvalidResizeRatio = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_err_image_format_cannot_be_compressed() {
        let filename = "test_image19.bmp";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let mut img = RusImg::open(path).unwrap();
        let result = img.compress(Some(50.0));
        assert!(result.is_err());
        if let Err(e) = result {
            if let RusimgError::ImageFormatCannotBeCompressed = e {
                // Expected error
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        } else {
            panic!("Expected an error, but got Ok");
        }
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_err_source_path_must_be_specified() {
        let filename = "test_image20.png";
        let width = 100;
        let height = 100;
        generate_test_image(filename, width, height);
        let path = Path::new(filename);
        let img = RusImg::open(path).unwrap();
        let result = img.get_input_filepath();
        assert!(result.is_ok());
        std::fs::remove_file(filename).unwrap();
    }
}
