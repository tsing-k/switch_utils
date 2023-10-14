use clap::Parser;
use shadow_rs::shadow;
shadow!(build);

const VERSION: &str = build::LAST_TAG;

const LONG_VERSION: &str = shadow_rs::formatcp!(r#"{}
Arch        : {}
Branch      : {}
Commit      : {}
Build time  : {}
Mode        : {}"#, 
build::LAST_TAG, 
build::BUILD_TARGET_ARCH, 
build::BRANCH,  
build::SHORT_COMMIT, 
build::BUILD_TIME,
build::BUILD_RUST_CHANNEL);

#[derive(Parser, Debug)]
#[command(author, version = VERSION, about, long_about = None, long_version = LONG_VERSION)]
pub struct Cli {
    /// 文件路径
    #[arg(name = "file")]
    pub file_name: Option<String>,
}
