pub mod error;
pub mod photo;

use std::fs;
use std::path::Path;
use crate::error::PhotoSortError;
use crate::photo::Photo;

pub fn sort_photos_by_install_date(photo_dir: &str, output_dir: &str) -> Result<(), PhotoSortError> {
    let mut photos = Vec::new();

    for entry in fs::read_dir(photo_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                if let Ok(photo) = Photo::new(path_str) {
                    photos.push(photo);
                }
            }
        }
    }

    photos.sort_by(|a, b| a.date.cmp(&b.date));

    for photo in photos {
        let date_dir = format!("{}/{}", output_dir, photo.date);
        fs::create_dir_all(&date_dir)?;
        
        let file_name = Path::new(&photo.path).file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| PhotoSortError::DateParseError("无效的文件名".to_string()))?;
            
        let dest_path = format!("{}/{}", date_dir, file_name);
        fs::copy(&photo.path, &dest_path)?;
    }

    Ok(())
}