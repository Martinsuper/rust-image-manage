use std::path::Path;
use std::fs;
use chrono::{DateTime, Local};
use crate::error::PhotoSortError;
use rexif::{ExifTag, TagValue, parse_file};
use log::{info, warn, error};

#[derive(Debug)]
pub struct Photo {
    pub path: String,
    pub date: String,
}

impl Photo {
    pub fn new(path: &str) -> Result<Self, PhotoSortError> {
        // 检查文件是否为支持的图片格式
        if (!Self::is_supported_image(path)) {
            warn!("跳过不支持的文件: {}", path);
            return Err(PhotoSortError::UnsupportedFormat(format!("不支持的文件格式: {}", path)));
        }

        let date = Self::extract_date(path)?;
        info!("✓ 成功解析照片 [{}]", path);
        Ok(Photo {
            path: path.to_string(),
            date,
        })
    }

    fn is_supported_image(path: &str) -> bool {
        let extensions = [
            // 常规图片格式
            "jpg", "jpeg", "png", "gif", "tiff",
            // RAW格式
            "arw",  // Sony
            "cr2", "cr3",  // Canon
            "nef",  // Nikon
            "orf",  // Olympus
            "rw2",  // Panasonic
            "pef",  // Pentax
            "raf",  // Fujifilm
            "raw", "dng"  // 通用RAW格式
        ];
        
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    fn extract_date(path: &str) -> Result<String, PhotoSortError> {
        // 首先尝试从EXIF获取日期
        if let Ok(date) = Self::get_exif_date(path) {
            return Ok(date);
        }

        // 如果EXIF不可用，使用文件创建时间
        Self::get_file_date(path)
    }

    fn get_exif_date(path: &str) -> Result<String, PhotoSortError> {
        let exif = parse_file(path).map_err(|e| {
            error!("❌ 解析EXIF失败 {}: {}", path, e);
            PhotoSortError::ExifError(e)
        })?;

        for entry in exif.entries {
            if entry.tag == ExifTag::DateTimeOriginal {
                if let TagValue::Ascii(values) = entry.value {
                    let date_str = values;
                        info!("📅 从EXIF提取到日期: {}", date_str);
                        return Ok(date_str.replace(':', "-").replace(' ', "_"));
                    }
                }
            }
        Err(PhotoSortError::DateParseError("无EXIF日期".to_string()))
    }

    fn get_file_date(path: &str) -> Result<String, PhotoSortError> {
        let metadata = fs::metadata(path)?;
        let created: DateTime<Local> = metadata.created()
            .map_err(|e| PhotoSortError::DateParseError(e.to_string()))?
            .into();
            
        let date_str = created.format("%Y-%m-%d_%H-%M-%S").to_string();
        info!("📅 使用文件创建时间: {} -> {}", path, date_str);
        Ok(date_str)
    }
}
