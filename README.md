# librusimg

An integrated image processing library for BMP, JPEG, PNG, and WebP formats.   

This library was developed for the [Rusimg](https://github.com/yotiosoft/rusimg) project, but it is open for use in other projects as well.

## Features

- Image Conversion (jpeg, png, webp, bmp)
- Set Conversion Quality
- Image Resizing
- Image Cropping
- Grayscale Conversion
- Save the image

### Image Conversion

Rusimg can convert images to the following formats.  

- For binary crates, the conversion format can be specified with the ``-c`` option.
- For library crates, the conversion format can be specified by calling the ``rusimg::RusImg.convert()`` function.

| format | backend library                             | library crate extension |
| ------ | ------------------------------------------- | ----------------------- |
| jpeg   | [mozjpeg](https://crates.io/crates/mozjpeg) | Extension::Jpeg         |
| png    | [oxipng](https://crates.io/crates/oxipng)   | Extension::Png          |
| webp   | [webp](https://crates.io/crates/webp)       | Extension::Webp         |
| bmp    | [image](https://crates.io/crates/image)     | Extension::Bmp          |


### Set Conversion Quality

Rusimg can set the quality of the converted image. This depends on each image format.

- For binary crates, the quality can be specified with the ``-q`` option. 
- For library crates, the quality can be specified by calling the ``rusimg::RusImg.compress()`` function.

| format | quality                                                      | note                                                         |
| ------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| jpeg   | 0-100                                                        | By default, the quality is set to 75.                        |
| png    | [0, 17.0], (17.0, 34.0], (34.0, 51.0], (51.0, 68.0], (68.0, 85.0], (85.0, 100.0] | Because the ``oxipng`` crate must be set to the 6 compression levels, input values will be converted into 6 levels. By default, the quality is set to 68.0-85.0. |
| webp   | 0-100                                                        | By default, the quality is set to 75.0.                      |
| bmp    | none                                                         | BMP does not have a quality setting because it is a lossless format. |

### Image Resizing

Resize images. The resize ratio is specified by a scaling factor (0, 100].  

The resize ratio can be specified by calling the ``rusimg::RusImg.resize()`` function.

### Image Cropping

Crop images.  

The crop size can be specified by calling the ``rusimg::RusImg.trim()`` or ``rusimg::RusImg.trim_rect()`` function.

### Grayscale Conversion

Convert images to grayscale.  

The grayscale conversion can be specified by calling the ``rusimg::RusImg.grayscale()`` function.

### Save the image

Save the image to the specified file path.

## Install

Use ``cargo`` to add the library crate.

```bash
$ cargo add rusimg
```

Or, add this to your ``Cargo.toml``.

```toml
[dependencies]
rusimg = "0.1.0"
```

If you don't use the specified image format, you can remove it from the features.  
For example, if don't use the webp format, leave ``webp`` out of the features so that the webp format is not included in the library.

```toml
[dependencies]
rusimg = { version = "0.1.0", default-features = false, features = ["bmp", "jpeg", "png"] }
```

## Typical functions

### rusimg::open_image()

Given a file path, open_image() returns struct RusImg, which contains the data for that image.
Struct ``RusImg`` has public processing functions for that image in ``RusimgTrait``.

```rust
pub fn open_image(path: &Path) -> Result<RusImg, RusimgError>;
```

### rusimg::RusImg.convert()

Converts the image to the specified format.  
If conversion is successful, the image data is updated in the struct RusImg.

```rust
pub fn convert(&mut self, new_extension: &Extension) -> Result<(), RusimgError>;
```

### rusimg::RusImg.save_image()

``save_image()`` saves the image to the specified file path.  
If the destination file path is not specified, the image is saved to the same file path as the source file (excluding the file extension).

```rust
pub fn save_image(&mut self, path: Option<&str>) -> Result<SaveStatus, RusimgError>;
```

## Add new image type & backend yourself

With implementing the ``RusimgTrait`` trait, you can add a new image format backend to librusimg that is not currently supported.  
``Extension::ExternalFormat(String)`` is provided for the library crate users to use if they wish to implement their own alternate image file format.

Example: Implementing ``my_bmp`` that implements ``bmp`` format myself.
```rust
mod my_bmp;
use librusimg::RusimgTrait;
use librusimg::RusImg;
use librusimg::Extension;
use librusimg::RusimgError;
use std::io::Read;
use std::path::Path;

pub fn open_my_bmp(path: &Path) -> Result<RusImg, RusimgError> {
    let mut raw_data = std::fs::File::open(&path.to_path_buf()).map_err(|e| RusimgError::FailedToOpenFile(e.to_string()))?;
    let mut buf = Vec::new();
    raw_data.read_to_end(&mut buf).map_err(|e| RusimgError::FailedToReadFile(e.to_string()))?;
    let metadata_input = raw_data.metadata().map_err(|e| RusimgError::FailedToGetMetadata(e.to_string()))?;

    let mybmpimg = my_bmp::MyBmpImage::open(path.to_path_buf(), buf, metadata_input)?;
    Ok(RusImg {
        extension: Extension::ExternalFormat("my_bmp".to_string()),
        data: Box::new(mybmpimg),
    })
}

fn main() {
    // get file path from command line
    let img_filepath = std::env::args().nth(1).unwrap();

    // open image "my_bmp"
    let my_img = open_my_bmp(&Path::new(&img_filepath));
    match my_img {
        Ok(mut my_img) => {
            my_img.resize(200).map_err(|e| println!("{}", e)).unwrap();
            my_img.save_image(Some("test_save2.bmp")).map_err(|e| println!("{}", e)).unwrap();         
        }
        Err(e) => println!("{}", e),
    }
}
```

In my_bmp.rs, implement the ``RusimgTrait`` trait for the bmp format.

```rust
use image::DynamicImage;

use std::fs::Metadata;
use std::path::PathBuf;

use rusimg::{ImgSize, RusimgError, RusimgTrait};

#[derive(Debug, Clone)]
pub struct MyBmpImage {
    pub image: DynamicImage,
    size: ImgSize,
    pub metadata_input: Metadata,
    pub metadata_output: Option<Metadata>,
    pub filepath_input: PathBuf,
    pub filepath_output: Option<PathBuf>,
}

impl RusimgTrait for MyBmpImage {
    fn import(image: DynamicImage, source_path: PathBuf, source_metadata: Metadata) -> Result<Self, RusimgError> {
        // create MyBmpImage object
        ...
    }
    fn open(path: PathBuf, image_buf: Vec<u8>, metadata: Metadata) -> Result<Self, RusimgError> {
        // open the image and create MyBmpImage object
        ...
    }
    fn save(&mut self, path: Option<PathBuf>) -> Result<(), RusimgError> {
        // save the image
        ...
    }
    ...
}
```

## Data types

### Trait

#### trait RusimgTrait

``RusimgTrait`` is a trait that contains the image processing functions.

```rust
pub trait RusimgTrait {
    /// Import an image from a DynamicImage object.
    fn import(image: DynamicImage, source_path: PathBuf, source_metadata: Metadata) -> Result<Self, RusimgError> where Self: Sized;
    /// Open an image from a image buffer.
    /// The ``path`` parameter is the file path of the image, but it is used for copying the file path to the object.
    /// This returns a RusImg object.
    fn open(path: PathBuf, image_buf: Vec<u8>, metadata: Metadata) -> Result<Self, RusimgError> where Self: Sized;
    /// Save the image to a file to the ``path``.
    fn save(&mut self, path: Option<PathBuf>) -> Result<(), RusimgError>;
    /// Compress the image with the quality parameter.
    fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError>;
    /// Resize the image with the resize_ratio parameter.
    fn resize(&mut self, resize_ratio: u8) -> Result<ImgSize, RusimgError>;
    /// Trim the image with the trim parameter.
    /// The trim parameter is a Rect object.
    fn trim(&mut self, trim: Rect) -> Result<ImgSize, RusimgError>;
    /// Grayscale the image.
    fn grayscale(&mut self);
    /// Set a image::DynamicImage to the image object.
    /// After setting the image, the image object will be updated.
    fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError>;
    /// Get a image::DynamicImage from the image object.
    fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError>;
    /// Get the source file path.
    fn get_source_filepath(&self) -> PathBuf;
    /// Get the destination file path.
    fn get_destination_filepath(&self) -> Option<PathBuf>;
    /// Get the source metadata.
    fn get_metadata_src(&self) -> Metadata;
    /// Get the destination metadata.
    fn get_metadata_dest(&self) -> Option<Metadata>;
    /// Get the image size.
    fn get_size(&self) -> ImgSize;
    /// Get a file path for saving an image.
    fn get_save_filepath(&self, source_filepath: &PathBuf, destination_filepath: Option<PathBuf>, new_extension: &String) -> Result<PathBuf, RusimgError>;
}
```

### Structs

#### struct RusImg

struct ``RusImg`` holds the file extension and the image data (``RusimgTrait``).  
``RusimgTrait`` is a trait that contains the image processing functions, but struct ``RusImg`` implements these wrapper functions.

```rust
pub struct RusImg {
    pub extension: Extension,
    pub data: Box<(dyn RusimgTrait)>,
}
```

##### struct RusImg implements

struct ``RusImg`` implements following functions.

```rust
impl RusImg {
    /// Get image size.
    pub fn get_image_size(&self) -> Result<ImgSize, RusimgError>;

    /// Resize an image.
    /// It must be called after open_image().
    /// Set ratio to 100 to keep the original size.
    pub fn resize(&mut self, ratio: u8) -> Result<ImgSize, RusimgError>;

    /// Trim an image. Set the trim area with four u32 values: x, y, w, h.
    /// It must be called after open_image().
    pub fn trim(&mut self, trim_x: u32, trim_y: u32, trim_w: u32, trim_h: u32) -> Result<ImgSize, RusimgError>;
    /// Trim an image. Set the trim area with a rusimg::Rect object.
    /// It must be called after open_image().
    pub fn trim_rect(&mut self, trim_area: Rect) -> Result<ImgSize, RusimgError>;

    /// Grayscale an image.
    /// It must be called after open_image().
    pub fn grayscale(&mut self) -> Result<(), RusimgError>;

    /// Compress an image.
    /// It must be called after open_image().
    /// Set quality to 100 to keep the original quality.
    pub fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError>;

    /// Convert an image to another format.
    /// And replace the original image with the new one.
    /// It must be called after open_image().
    pub fn convert(&mut self, new_extension: &Extension) -> Result<(), RusimgError>;

    /// Set a DynamicImage to an Img.
    pub fn set_dynamic_image(&mut self, image: DynamicImage) -> Result<(), RusimgError>;

    /// Get a DynamicImage from an Img.
    pub fn get_dynamic_image(&mut self) -> Result<DynamicImage, RusimgError>;

    /// Get file extension.
    pub fn get_extension(&self) -> Extension;

    /// Get input file path.
    pub fn get_input_filepath(&self) -> PathBuf;

    /// Save an image to a file.
    /// If path is None, the original file will be overwritten.
    pub fn save_image(&mut self, path: Option<&str>) -> Result<SaveStatus, RusimgError>;
}
```

#### Rect

Struct ``Rect`` is used to specify the crop area.  
``rusimg::RusImg.trim_rect()`` needs a ``Rect`` object to specify the crop area.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
```

#### ImgSize

Struct ``ImgSize`` is used to get the image size.  
``rusimg::RusImg.get_image_size()``, ``rusimg::RusImg.resize()``, ``rusimg::RusImg.trim()``, and ``rusimg::RusImg.trim_rect()`` return this struct.

```rust
#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct ImgSize {
    pub width: usize,
    pub height: usize,
}
```

#### SaveStatus

Struct ``SaveStatus`` is used for tracking the status of saving an image.  
It contains the output file path, the file size before saving, and the file size after saving.  
If the image has compression, the file size after saving will be different from the file size before saving.  
``rusimg::RusImg.save_image()`` returns this enum.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct SaveStatus {
    pub output_path: Option<PathBuf>,
    pub before_filesize: u64,
    pub after_filesize: Option<u64>,
}
```

### Enum

#### Extension

Enum ``Extension`` indicates the file extension.  
ExternalFormat(String) is provided for the library crate users to use if they wish to implement their own alternate image file format.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Extension {
    Bmp,
    Jpeg,
    Png,
    Webp,
    ExternalFormat(String),
}
impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Extension::Bmp => write!(f, "bmp"),
            Extension::Jpeg => write!(f, "jpeg"),
            Extension::Png => write!(f, "png"),
            Extension::Webp => write!(f, "webp"),
            Extension::ExternalFormat(s) => write!(f, "{}", s),
        }
    }
}
```
