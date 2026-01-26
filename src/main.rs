use cp_unfold::{Config, Unfold};
use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

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

fn run() -> Result<()> {
    let args = Args::parse();

    // 設定を初期化して実行時設定を取得
    let runtime_config = Config::initialize_and_merge(
        args.src,
        args.library_name,
        args.file_dir,
        args.library_path,
    )?;

    // Unfoldを実行
    let mut unfold = Unfold::from_args(
        runtime_config.src,
        runtime_config.library_name,
        runtime_config.file_dir,
        runtime_config.library_path,
    );

    let output = unfold.unfold()?;
    println!("{}", output);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
