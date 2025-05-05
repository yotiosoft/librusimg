use mozjpeg::{Compress, ColorSpace, ScanMode};
use image::DynamicImage;

use std::fs::Metadata;
use std::io::Write;
use std::path::PathBuf;

use super::super::{BackendTrait, RusimgError, ImgSize, Rect};

#[derive(Debug, Clone)]
pub struct JpegImage {
    pub image: DynamicImage,
    image_bytes: Option<Vec<u8>>,
    size: ImgSize,
    operations_count: u32,
    extension_str: String,
    pub metadata_input: Option<Metadata>,
    pub metadata_output: Option<Metadata>,
    pub filepath_input: Option<PathBuf>,
    pub filepath_output: Option<PathBuf>,
}

fn remove_alpha_channel(image: DynamicImage) -> DynamicImage {
    // Remove alpha channel if it exists because JPEG does not support it
    if image.color() == image::ColorType::Rgba8 {
        return DynamicImage::ImageRgb8(image.to_rgb8());
    }
    if image.color() == image::ColorType::Rgba16 {
        return DynamicImage::ImageRgb16(image.to_rgb16());
    }
    if image.color() == image::ColorType::Rgba32F {
        return DynamicImage::ImageRgb32F(image.to_rgb32f());
    }
    image
}

impl BackendTrait for JpegImage {
    /// Import an image from a DynamicImage object.
    fn import(image: Option<DynamicImage>, source_path: Option<PathBuf>, source_metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        let image = image.ok_or(RusimgError::ImageNotSpecified)?;
        let size = ImgSize { width: image.width() as usize, height: image.height() as usize };

        let image = remove_alpha_channel(image); // Remove alpha channel if it exists because JPEG does not support it

        Ok(Self {
            image,
            image_bytes: None,
            size,
            operations_count: 0,
            extension_str: "jpg".to_string(),
            metadata_input: source_metadata,
            metadata_output: None,
            filepath_input: source_path,
            filepath_output: None,
        })
    }

    /// Open an image from a image buffer.
    fn open(path: Option<PathBuf>, image_buf: Option<Vec<u8>>, metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        let path = path.ok_or(RusimgError::ImageNotSpecified)?; // 画像のパスが指定されていない場合はエラー
        let image_buf = image_buf.ok_or(RusimgError::ImageNotSpecified)?; // 画像のバイナリデータが指定されていない場合はエラー
        let metadata = metadata.ok_or(RusimgError::ImageNotSpecified)?; // 画像のメタデータが指定されていない場合はエラー
        
        let image = image::load_from_memory(&image_buf).map_err(|e| RusimgError::FailedToOpenImage(e.to_string()))?;
        let size = ImgSize { width: image.width() as usize, height: image.height() as usize };

        let extension_str = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();

        let image = remove_alpha_channel(image); // Remove alpha channel if it exists because JPEG does not support it
        
        Ok(Self {
            image,
            image_bytes: None,
            size,
            operations_count: 0,
            extension_str,
            metadata_input: Some(metadata),
            metadata_output: None,
            filepath_input: Some(path),
            filepath_output: None,
        })
    }

    /// Save the image to a file.
    fn save(&mut self, path: Option<PathBuf>) -> Result<(), RusimgError> {
        let save_path = Self::get_save_filepath(&self, &self.filepath_input, path, &self.extension_str)?;

        // image_bytes == None の場合、DynamicImage を 保存
        if self.image_bytes.is_none() {
            self.image.to_rgba8().save(&save_path).map_err(|e| RusimgError::FailedToSaveImage(e.to_string()))?;
            self.metadata_output = Some(std::fs::metadata(&save_path).map_err(|e| RusimgError::FailedToGetMetadata(e.to_string()))?);
        }
        // image_bytes != None の場合、mozjpeg::Compress で圧縮したバイナリデータを保存
        else {
            let mut file = std::fs::File::create(&save_path).map_err(|e| RusimgError::FailedToCreateFile(e.to_string()))?;
            file.write_all(&self.image_bytes.as_ref().unwrap()).map_err(|e| RusimgError::FailedToWriteFIle(e.to_string()))?;
            self.metadata_output = Some(file.metadata().map_err(|e| RusimgError::FailedToGetMetadata(e.to_string()))?);
        }

        self.filepath_output = Some(save_path);

        Ok(())
    }

    /// Compress the image.
    /// quality: Option<f32> 0.0 - 100.0
    fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError> {
        let quality = quality.unwrap_or(75.0);  // default quality: 75.0

        let image_bytes = self.image.clone().into_bytes();

        let mut compress = Compress::new(ColorSpace::JCS_RGB);
        compress.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        compress.set_size(self.size.width, self.size.height);
        compress.set_quality(quality);
        let comp = compress.start_compress(image_bytes).map_err(|e| RusimgError::FailedToCompressImage(Some(e.to_string())))?;

        self.image_bytes = Some(comp.finish().map_err(|e| RusimgError::FailedToCompressImage(Some(e.to_string())))?);

        self.operations_count += 1;

        Ok(())
    }

    /// Resize the image.
    /// Set the resize_ratio between 1 and 100.
    fn resize(&mut self, resize_ratio: u8) -> Result<ImgSize, RusimgError> {
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
        let image = remove_alpha_channel(image); // Remove alpha channel if it exists because JPEG does not support it
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
