use clap::Parser;

mod file;
use file::*;

mod cli;
use cli::Cli;

mod utils;

fn main() {
    let args = Cli::parse();
    let file;

    if let Some(filename) = args.file_name {
        // 指定文件
        file = DownloadFileType::open(filename.as_str());
    } else {
        // 未指定文件，需要遍历当前路径，找出可能要升级的文件
        file = get_file(".");
    }

    match file {
        Ok(file) => {
            if let Err(e) = download(file) {
                eprintln!("{e}");
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
