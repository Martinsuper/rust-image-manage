pub mod error;
pub mod photo;

use std::fs;
use std::path::Path;
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, error};
use crate::error::PhotoSortError;
use crate::photo::Photo;

pub fn show_progress(current: usize, total: usize, operation: &str) {
    let percentage = (current as f32 / total as f32) * 100.0;
    info!("🔄 {} 进度: [{}/{}] {:.1}%", operation, current, total, percentage);
}

pub fn sort_photos_by_install_date(photo_dir: &str, output_dir: &str) -> Result<(), PhotoSortError> {
    info!("开始处理照片目录: {}", photo_dir);
    
    // 确保输出目录存在
    fs::create_dir_all(output_dir)?;
    
    // 递归获取所有文件
    let mut entries = vec![];
    visit_dirs(Path::new(photo_dir), &mut entries)?;

    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-")
    );

    let output_dir = Arc::new(output_dir.to_string());
    
    // 并行处理照片
    entries.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                match Photo::new(path_str) {
                    Ok(photo) => Some(photo),
                    Err(e) => {
                        error!("处理文件失败 {}: {}", path_str, e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .try_for_each(|photo| -> Result<(), PhotoSortError> {
            let output_dir = Arc::clone(&output_dir);
            
            // 从源路径中提取年份和日期目录结构
            let path = Path::new(&photo.path);

            // 构建目标目录路径
            info!("输出目录：{}", photo.date);
            let target_dir = format!("{}/{}", *output_dir, photo.date);

            // 确保目标目录存在
            fs::create_dir_all(&target_dir)?;
            
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| PhotoSortError::ProcessError("无效的文件名".to_string()))?;
                
            let dest_path = format!("{}/{}", target_dir, file_name);
            info!("复制文件: {} -> {}", photo.path, dest_path);
            fs::copy(&photo.path, &dest_path)?;
            
            pb.inc(1);
            Ok(())
        })?;

    pb.finish_with_message("照片处理完成");
    info!("照片分类完成");
    Ok(())
}

// 新增递归遍历目录函数
fn visit_dirs(dir: &Path, entries: &mut Vec<fs::DirEntry>) -> Result<(), PhotoSortError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, entries)?;
            } else {
                entries.push(entry);
            }
        }
    }
    Ok(())
}