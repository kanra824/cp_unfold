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
    let mut config = Config::load();
    let config_exists = Config::exists();

    // 設定ファイルが存在せず、CLI引数もない場合は対話的に初期化
    if !config_exists && args.file_dir.is_none() && config.file_dir.is_none() {
        config = Config::interactive_init(args.library_name.clone(), args.library_path.clone());
        
        if let Err(e) = config.save() {
            eprintln!("Warning: Could not save config file: {}", e);
        } else {
            eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
        }
    }
    // CLI引数で設定が指定された場合も保存（初回のみ）
    else if !config_exists && (args.file_dir.is_some() || args.library_name.is_some() || args.library_path.is_some()) {
        let new_config = Config::from_cli_args(
            args.file_dir.as_ref(),
            args.library_name.clone(),
            args.library_path.as_ref(),
        );
        
        if let Err(e) = new_config.save() {
            eprintln!("Warning: Could not save config file: {}", e);
        } else {
            eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
        }
        
        config = new_config;
    }

    // CLI引数と設定ファイルをマージして実行時設定を作成
    let runtime_config = match config.merge_with_cli(
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
