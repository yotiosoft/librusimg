use image::DynamicImage;

use std::fs::Metadata;
use std::path::PathBuf;

use super::super::{ImgSize, RusimgError, BackendTrait, Rect};

#[derive(Debug, Clone)]
pub struct EmptyImage {
    pub image: Option<DynamicImage>,
    size: Option<ImgSize>,
    source_path: Option<PathBuf>,
}

impl BackendTrait for EmptyImage {
    /// Import an image from a DynamicImage object.
    fn import(image: Option<DynamicImage>, source_path: Option<PathBuf>, _source_metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        if image.is_some() {
            let image = image.unwrap();
            let size = ImgSize { width: image.width() as usize, height: image.height() as usize };
            Ok(Self {
                image: Some(image),
                size: Some(size),
                source_path,
            })
        } else {
            Ok(Self {
                image: None,
                size: None,
                source_path,
            })
        }
    }

    /// Open an image from a image buffer.
    fn open(_path: Option<PathBuf>, _image_buf: Option<Vec<u8>>, _metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        // Because this is an empty image, we don't need to open anything.
        Err(RusimgError::UnsupportedFeature)
    }

    /// Save the image to a file.
    fn save(&mut self, _path: Option<PathBuf>) -> Result<(), RusimgError> {
        // Because this is an empty image, we don't need to save anything.
        // You must convert the image to another format before saving.
        Err(RusimgError::UnsupportedFeature)
    }

    /// Compressing a BMP image is not supported because BMP is a lossless format.
    fn compress(&mut self, _quality: Option<f32>) -> Result<(), RusimgError> {
        Err(RusimgError::ImageFormatCannotBeCompressed)
    }

    /// Resize the image.
    /// Set the resize_ratio between 1 and 100.
    fn resize(&mut self, resize_ratio: u8) -> Result<ImgSize, RusimgError> {
        if self.image.is_none() {
            return Err(RusimgError::ImageNotSpecified);
        }
        if self.size.is_none() {
            return Err(RusimgError::ImageNotSpecified);
        }

        let nwidth = (self.size.unwrap().width as f32 * (resize_ratio as f32 / 100.0)) as usize;
        let nheight = (self.size.unwrap().height as f32 * (resize_ratio as f32 / 100.0)) as usize;
        
        self.image = Some(self.image.clone().unwrap().resize(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3));

        self.size.unwrap().width = nwidth;
        self.size.unwrap().height = nheight;

        Ok(self.size.unwrap())
    }

    /// Trim the image.
    /// Set the trim area with the rusimg::Rect structure.
    fn trim(&mut self, trim: Rect) -> Result<ImgSize, RusimgError> {
        if self.image.is_none() {
            return Err(RusimgError::ImageNotSpecified);
        }
        if self.size.is_none() {
            return Err(RusimgError::ImageNotSpecified);
        }

        let mut w = trim.w;
        let mut h = trim.h;
        if self.size.unwrap().width < (trim.x + trim.w) as usize || self.size.unwrap().height < (trim.y + trim.h) as usize {
            if self.size.unwrap().width > trim.x as usize && self.size.unwrap().height > trim.y as usize {
                w = if self.size.unwrap().width < (trim.x + trim.w) as usize { self.size.unwrap().width as u32 - trim.x } else { trim.w };
                h = if self.size.unwrap().height < (trim.y + trim.h) as usize { self.size.unwrap().height as u32 - trim.y } else { trim.h };
                //println!("Required width or height is larger than image size. Corrected size: {}x{} -> {}x{}", trim_wh.0, trim_wh.1, w, h);
            }
            else {
                return Err(RusimgError::InvalidTrimXY);
            }
        }

        self.image = Some(self.image.clone().unwrap().crop(trim.x, trim.y, w, h));

        self.size = Some(ImgSize { width: w as usize, height: h as usize });

        Ok(self.size.unwrap())
    }

    /// Convert the image to grayscale.
    fn grayscale(&mut self) {
        self.image = Some(self.image.clone().unwrap().grayscale());
    }

    /// Set the image to a DynamicImage object.
    fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError> {
        self.image = Some(image);
        Ok(())
    }
    
    /// Get the DynamicImage object.
    fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError> {
        Ok(self.image.clone().unwrap())
    }

    /// Get the source file path.
    fn get_source_filepath(&self) -> Option<PathBuf> {
        None
    }

    /// Get the destination file path.
    fn get_destination_filepath(&self) -> Result<Option<PathBuf>, RusimgError> {
        Err(RusimgError::UnsupportedFeature)
    }

    /// Get the source metadata.
    fn get_metadata_src(&self) -> Option<Metadata> {
        None
    }

    /// Get the destination metadata.
    fn get_metadata_dest(&self) -> Option<Metadata> {
        None
    }

    /// Get the image size.
    fn get_size(&self) -> Result<ImgSize, RusimgError> {
        if self.size.is_none() {
            return Err(RusimgError::ImageNotSpecified);
        }
        Ok(self.size.unwrap())
    }
}
