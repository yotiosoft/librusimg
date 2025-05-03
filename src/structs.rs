use std::path::PathBuf;

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
