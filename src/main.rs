mod unfold;
mod config;

use unfold::Unfold;
use config::Config;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cp_unfold")]
#[command(about = "Unfold Rust source files by expanding imports", long_about = None)]
struct Args {
    /// Source file name to unfold
    #[arg(short, long)]
    src: Option<String>,

    /// Library import name (e.g., "library")
    #[arg(short, long)]
    library_name: Option<String>,

    /// Directory containing the source file
    #[arg(short, long)]
    file_dir: Option<PathBuf>,

    /// Path to the library directory
    #[arg(short = 'p', long)]
    library_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // 設定を初期化して実行時設定を取得
    let runtime_config = match Config::initialize_and_merge(
        args.src,
        args.library_name,
        args.file_dir,
        args.library_path,
    ) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Unfoldを実行
    let mut unfold = Unfold::from_args(
        runtime_config.src,
        runtime_config.library_name,
        runtime_config.file_dir,
        runtime_config.library_path,
    );

    match unfold.unfold() {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
