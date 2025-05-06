use jpeg_encoder::{Encoder, ColorType};
use image::DynamicImage;

use std::fs::Metadata;
use std::path::PathBuf;

use super::super::{BackendTrait, RusimgError, ImgSize, Rect};

#[derive(Debug, Clone)]
pub struct JpegImage {
    pub image: DynamicImage,
    size: ImgSize,
    operations_count: u32,
    extension_str: String,
    required_quality: Option<f32>,
    pub metadata_input: Option<Metadata>,
    pub metadata_output: Option<Metadata>,
    pub filepath_input: Option<PathBuf>,
    pub filepath_output: Option<PathBuf>,
}

impl BackendTrait for JpegImage {
    /// Import an image from a DynamicImage object.
    fn import(image: Option<DynamicImage>, source_path: Option<PathBuf>, source_metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        let image = image.ok_or(RusimgError::ImageNotSpecified)?;
        let size = ImgSize { width: image.width() as usize, height: image.height() as usize };

        Ok(Self {
            image,
            size,
            operations_count: 0,
            extension_str: "jpg".to_string(),
            required_quality: None,
            metadata_input: source_metadata,
            metadata_output: None,
            filepath_input: source_path,
            filepath_output: None,
        })
    }

    /// Open an image from a image buffer.
    fn open(path: Option<PathBuf>, image_buf: Option<Vec<u8>>, metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        let path = path.ok_or(RusimgError::ImageNotSpecified)?; // If the image path is not specified, return an error.
        let image_buf = image_buf.ok_or(RusimgError::ImageNotSpecified)?; // If the image buffer is not specified, return an error.
        let metadata = metadata.ok_or(RusimgError::ImageNotSpecified)?; // If the metadata is not specified, return an error.
        
        let image = image::load_from_memory(&image_buf).map_err(|e| RusimgError::FailedToOpenImage(e.to_string()))?;
        let size = ImgSize { width: image.width() as usize, height: image.height() as usize };

        let extension_str = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
        
        Ok(Self {
            image,
            size,
            operations_count: 0,
            extension_str,
            required_quality: None,
            metadata_input: Some(metadata),
            metadata_output: None,
            filepath_input: Some(path),
            filepath_output: None,
        })
    }

    /// Save the image to a file.
    fn save(&mut self, path: Option<PathBuf>) -> Result<(), RusimgError> {
        let save_path = Self::get_save_filepath(&self, &self.filepath_input, path, &self.extension_str)?;

        // If compression is not specified, set the default quality to 75.0
        let quality = if let Some(quality) = self.required_quality {
            quality
        } else {
            75.0
        };
        let encoder = Encoder::new_file(&save_path, quality as u8).map_err(|e| RusimgError::FailedToCreateFile(e.to_string()))?;
        encoder.encode(&self.image.to_rgb8(), self.size.width as u16, self.size.height as u16, ColorType::Rgb).map_err(|e| RusimgError::FailedToSaveImage(e.to_string()))?;
        self.metadata_output = Some(std::fs::metadata(&save_path).map_err(|e| RusimgError::FailedToGetMetadata(e.to_string()))?);

        self.filepath_output = Some(save_path);

        Ok(())
    }

    /// Compress the image.
    /// quality: Option<f32> 0.0 - 100.0
    /// Because the jpeg_encoder crate compresses the image when saving it, the compress() method does not need to do anything.
    /// So this method only sets the quality value.
    fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError> {
        let quality = quality.unwrap_or(75.0);  // default quality: 75.0
        self.required_quality = Some(quality);
        self.operations_count += 1;
        Ok(())
    }

    /// Resize the image.
    fn resize(&mut self, resize_ratio: f32) -> Result<ImgSize, RusimgError> {
        let nwidth = (self.size.width as f32 * (resize_ratio as f32 / 100.0)) as usize;
        let nheight = (self.size.height as f32 * (resize_ratio as f32 / 100.0)) as usize;
        
        self.image = self.image.resize(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3);

        self.size.width = nwidth;
        self.size.height = nheight;

        self.operations_count += 1;
        Ok(self.size)
    }

    /// Trim the image.
    /// trim: rusimg::Rect { x: u32, y: u32, w: u32, h: u32 }
    fn trim(&mut self, trim: Rect) -> Result<ImgSize, RusimgError> {
        let mut w = trim.w;
        let mut h = trim.h;
        if self.size.width < (trim.x + trim.w) as usize || self.size.height < (trim.y + trim.h) as usize {
            if self.size.width > trim.x as usize && self.size.height > trim.y as usize {
                w = if self.size.width < (trim.x + trim.w) as usize { self.size.width as u32 - trim.x } else { trim.w };
                h = if self.size.height < (trim.y + trim.h) as usize { self.size.height as u32 - trim.y } else { trim.h };
                //println!("Required width or height is larger than image size. Corrected size: {}x{} -> {}x{}", trim_wh.0, trim_wh.1, w, h);
            }
            else {
                return Err(RusimgError::InvalidTrimXY);
            }
        }

        self.image = self.image.crop(trim.x, trim.y, w, h);

        self.size.width = w as usize;
        self.size.height = h as usize;

        Ok(self.size)
    }

    /// Convert the image to grayscale.
    fn grayscale(&mut self) {
        self.image = self.image.grayscale();
        self.operations_count += 1;
    }

    /// Set the image to a DynamicImage object.
    fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError> {
        self.image = image;
        Ok(())
    }

    /// Get the DynamicImage object.
    fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError> {
        Ok(self.image.clone())
    }

    /// Get the source file path.
    fn get_source_filepath(&self) -> Option<PathBuf> {
        self.filepath_input.clone()
    }

    /// Get the destination file path.
    fn get_destination_filepath(&self) -> Result<Option<PathBuf>, RusimgError> {
        Ok(self.filepath_output.clone())
    }

    /// Get the source metadata.
    fn get_metadata_src(&self) -> Option<Metadata> {
        self.metadata_input.clone()
    }

    /// Get the destination metadata.
    fn get_metadata_dest(&self) -> Option<Metadata> {
        self.metadata_output.clone()
    }

    /// Get the image size.
    fn get_size(&self) -> Result<ImgSize, RusimgError> {
        Ok(self.size)
    }
}
