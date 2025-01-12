use clap::Parser;
use env_logger::Env;
use log::{error, info};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// 照片目录
    photo_dir: String,
    /// 分类后拷贝的目录
    output_dir: String,
}

fn main() {
    // 初始化日志
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // let args: Args = Args::parse();
    // info!("开始处理照片");
    // info!("源目录: {}", args.photo_dir);
    // info!("目标目录: {}", args.output_dir);

    if let Err(e) = photo_sorter::sort_photos_by_install_date("/Users/duanluyao/Documents/Sony/", "/Users/duanluyao/Documents/照片") {
        error!("处理照片失败: {}", e);
        std::process::exit(1);
    }
    
    info!("照片处理完成");
}