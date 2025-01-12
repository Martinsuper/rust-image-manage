use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// 照片目录
    photo_dir: String,
    /// 分类后拷贝的目录
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    // 调用分类功能
    match photo_sorter::sort_photos_by_install_date(&args.photo_dir, &args.output_dir) {
        Ok(_) => println!("照片分类成功。"),
        Err(e) => eprintln!("分类过程中发生错误: {}", e),
    }
}