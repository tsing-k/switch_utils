use clap::Parser;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{error, info, LevelFilter};
use anyhow::{Result, Error};

mod file;
use file::*;

mod cli;
use cli::Cli;

mod utils;

fn log_init() -> Result<()> {
    let log_path = "./log/log_test.log";
    let archive_path = "./archive/log_test_{}.log";
    let log_pattern = "[{d(%Y-%m-%d %H:%M:%S %Z)}][{f}:{L}][{l}]: {m}{n}";

    let trigger = SizeTrigger::new(0x6400000);  // 100M
    let roller = FixedWindowRoller::builder()
        .base(1)
        .build(archive_path, 10)?;

    let requests = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .append(true)
        .build(log_path, Box::new(CompoundPolicy::new(
            Box::new(trigger),
            Box::new(roller))))?;

    let config = Config::builder()
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(Root::builder().appender("requests").build(LevelFilter::Debug))?;

    log4rs::init_config(config)?;

    Ok(())
}

fn main() {
    match log_init() {
        Ok(_) => {},
        Err(e) => { eprintln!("log module init failed. error: {e}") },
    }

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
                error!("{e}");
            }
            println!("download success");
            info!("download success");
        }
        Err(e) => {
            eprintln!("{e}");
            error!("{e}");
        }
    }
}
