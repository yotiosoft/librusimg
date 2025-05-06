# librusimg

![Crates.io Version](https://img.shields.io/crates/v/librusimg)
[![Rust](https://github.com/yotiosoft/librusimg/actions/workflows/rust.yml/badge.svg)](https://github.com/yotiosoft/librusimg/actions/workflows/rust.yml)

An integrated image processing, conversion, and compression library for BMP, JPEG, PNG, and WebP formats for Rust.

This library was developed for the [Rusimg](https://crates.io/crates/rusimg) project, but it is open for use in other projects as well.

## Install

Use ``cargo`` to add the library crate.

```bash
$ cargo add librusimg
```

Or, add this to your project's ``Cargo.toml``.

```toml
[dependencies]
librusimg = "0.1.1"
```

If you don't use the specified image format, you can remove it from the features.  
For example, if you don't use the webp format, leave ``webp`` out of the features so that the webp format is not included in the library.

```toml
[dependencies]
librusimg = { version = "0.1.1", default-features = false, features = ["bmp", "jpeg", "png"] }
```

## Features

- Open Image (bmp, jpeg, png, webp)
- Image Conversion (jpeg, png, webp, bmp)
- Image compression (jpeg, png, webp)
- Image Resizing
- Image Cropping
- Grayscale Conversion
- Save the image

### Open an image

After opening the image, the function will return a ``RusImg`` object.

```rust
pub fn open(path: &Path) -> Result<RusImg, RusimgError>;
```

### Generate a new image

You can create a new image from a ``DynamicImage`` object.

```rust
pub fn new(extension: &Extension, image: DynamicImage) -> Result<RusImg, RusimgError>;
```

See the [Image Conversion](#image-conversion) section for the supported extensions.

### Image Conversion

Rusimg can convert images to the following formats.  

The conversion format can be specified by calling the ``rusimg::RusImg.convert()`` function.

```rust
pub fn convert(&mut self, new_extension: &Extension) -> Result<(), RusimgError>;
```

| format | backend library                                       | library crate extension              |
| ------ | ----------------------------------------------------- | ------------------------------------ |
| jpeg   | [jpeg-encoder](https://crates.io/crates/jpeg-encoder) | Extension::Jpeg or Extension::Jpg *  |
| png    | [oxipng](https://crates.io/crates/oxipng)             | Extension::Png                       |
| webp   | [webp](https://crates.io/crates/webp)                 | Extension::Webp                      |
| bmp    | [image](https://crates.io/crates/image)               | Extension::Bmp                       |

\* The ``rusimg::Extension::Jpeg`` and ``rusimg::Extension::Jpg`` are the same, but file names will be saved as ``.jpeg`` and ``.jpg`` respectively.

### Image compression

Rusimg can set the quality of the converted image. This depends on each image format.  

The quality can be specified by calling the ``rusimg::RusImg.compress()`` function.

```rust
pub fn compress(&mut self, quality: Option<f32>) -> Result<(), RusimgError>;
```

| format | quality                                                      | note                                                         |
| ------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| jpeg   | 0-100                                                        | By default, the quality is set to 75.                        |
| png    | [0, 17.0], (17.0, 34.0], (34.0, 51.0], (51.0, 68.0], (68.0, 85.0], (85.0, 100.0] | Because the ``oxipng`` crate must be set to the 6 compression levels, input values will be converted into 6 levels. By default, the quality is set to 68.0-85.0. |
| webp   | 0-100                                                        | By default, the quality is set to 75.0.                      |
| bmp    | none                                                         | BMP does not have a quality setting because it is a lossless format. |

### Image Resizing

Resize images.  

The resize ratio can be specified by calling the ``rusimg::RusImg.resize()`` function.

```rust
pub fn resize(&mut self, ratio: f32) -> Result<ImgSize, RusimgError>;
```

### Image Cropping

Crop images.  

The crop size can be specified by calling the ``rusimg::RusImg.trim()`` or ``rusimg::RusImg.trim_rect()`` function.

```rust
pub fn trim(&mut self, trim_x: u32, trim_y: u32, trim_w: u32, trim_h: u32) -> Result<ImgSize, RusimgError>;
pub fn trim_rect(&mut self, trim_area: Rect) -> Result<ImgSize, RusimgError>;
```

### Grayscale Conversion

Convert images to grayscale.  

The grayscale conversion can be specified by calling the ``rusimg::RusImg.grayscale()`` function.

```rust
pub fn grayscale(&mut self) -> Result<(), RusimgError>;
```

### Save the image

Save the image to the specified file path.  
If the destination file path is not specified, the image is saved to the same file path as the source file (excluding the file extension).

```rust
pub fn save_image(&mut self, path: Option<&str>) -> Result<SaveStatus, RusimgError>;
```

## Want to try it out?

[rusimg](https://crates.io/crates/rusimg) is a command line tool that uses this library.
You can try it out by running the following command.

```bash
$ cargo install rusimg
```
Then, you can run the following command to convert an image.

```bash
$ rusimg -i <input_file> -o <output_file_or_directory> -c <format_to_convert> -q <quality> -r <resize_ratio>
```

## Add new image type & backend yourself

With implementing the ``BackendTrait`` trait, you can add a new image format backend to librusimg that is not currently supported.  
``Extension::ExternalFormat(String)`` is provided for the library crate users to use if they wish to implement their own alternate image file format.
Use the ``rusimg::RusImg.assemble()`` function to create a new RusImg object from the external format.

Example: Implementing ``my_bmp`` that implements ``bmp`` format myself.
```rust
mod my_bmp;
use librusimg::BackendTrait;
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

    let my_bmp_img = my_bmp::MyBmpImage::open(Some(path.to_path_buf()), Some(buf), Some(metadata_input))?;
    RusImg::assemble(
        &Extension::ExternalFormat("my_bmp".to_string()),
        Box::new(my_bmp_img),
    )
}

fn main() {
    // get file path from command line
    let img_filepath = std::env::args().nth(1).unwrap();

    // open image "my_bmp"
    let my_img = open_my_bmp(&Path::new(&img_filepath));
    match my_img {
        Ok(mut my_img) => {
            my_img.resize(200).map_err(|e| println!("{}", e)).unwrap();
            my_img.save_image(Some("test_save2.my.bmp")).map_err(|e| println!("{}", e)).unwrap();         
        }
        Err(e) => println!("{}", e),
    }
}
```

In my_bmp.rs, implement the ``BackendTrait`` trait for the bmp format.

```rust
use image::DynamicImage;

use std::fs::Metadata;
use std::path::PathBuf;

use librusimg::{ImgSize, RusimgError, BackendTrait};

#[derive(Debug, Clone)]
pub struct MyBmpImage {
    pub image: DynamicImage,
    size: ImgSize,
    pub metadata_input: Metadata,
    pub metadata_output: Option<Metadata>,
    pub filepath_input: PathBuf,
    pub filepath_output: Option<PathBuf>,
}

impl BackendTrait for MyBmpImage {
    fn import(image: Option<DynamicImage>, source_path: Option<PathBuf>, source_metadata: Option<Metadata>) -> Result<Self, RusimgError> {
        // create MyBmpImage object
        ...
    }
    fn open(path: Option<PathBuf>, image_buf: Option<Vec<u8>>, metadata: Option<Metadata>) -> Result<Self, RusimgError> {
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

## License

This project is licensed under the MIT License or Apache License 2.0.  
- For the MIT License, see [LICENSE-MIT](LICENSE-MIT) file.
- For the Apache License 2.0, see [LICENSE-APACHE](LICENSE-APACHE2.0) file.
