use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PhotoSortError {
    IoError(std::io::Error),
    ExifError(rexif::ExifError),
    DateParseError(String),
    UnsupportedFormat(String),
    ProcessError(String),
}

impl fmt::Display for PhotoSortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PhotoSortError::IoError(e) => write!(f, "IO错误: {}", e),
            PhotoSortError::ExifError(e) => write!(f, "EXIF解析错误: {}", e),
            PhotoSortError::DateParseError(e) => write!(f, "日期解析错误: {}", e),
            PhotoSortError::UnsupportedFormat(e) => write!(f, "不支持的格式: {}", e),
            PhotoSortError::ProcessError(e) => write!(f, "处理错误: {}", e),
        }
    }
}

impl Error for PhotoSortError {}

impl From<std::io::Error> for PhotoSortError {
    fn from(err: std::io::Error) -> PhotoSortError {
        PhotoSortError::IoError(err)
    }
}

impl From<rexif::ExifError> for PhotoSortError {
    fn from(err: rexif::ExifError) -> PhotoSortError {
        PhotoSortError::ExifError(err)
    }
}
