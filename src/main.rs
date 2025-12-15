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

    // 設定ファイルが存在せず、file_dir が指定されていない場合は標準入力から取得
    if !config_exists && args.file_dir.is_none() && config.file_dir.is_none() {
        use std::io::{self, Write};

        eprint!("Enter file directory (source file location): ");
        io::stderr().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let file_dir_input = input.trim().to_string();

        if file_dir_input.is_empty() {
            eprintln!("Error: file_dir cannot be empty");
            std::process::exit(1);
        }

        // 設定を作成して保存
        let new_config = Config {
            file_dir: Some(file_dir_input.clone()),
            library_name: args.library_name.clone(),
            library_path: args.library_path.as_ref().map(|p| p.display().to_string()),
        };
        
        if let Err(e) = new_config.save() {
            eprintln!("Warning: Could not save config file: {}", e);
        } else {
            eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
        }
        
        config = new_config;
    }

    // 優先順位: CLI引数 > 設定ファイル > デフォルト値
    let src = args.src.clone()
        .or_else(|| config.file_dir.as_ref().map(|_| "main.rs".to_string()))
        .unwrap_or_else(|| "main.rs".to_string());

    let library_name = args.library_name.clone()
        .or(config.library_name.clone())
        .unwrap_or_else(|| "library".to_string());

    let file_dir = args.file_dir.clone()
        .or_else(|| config.file_dir.as_ref().map(PathBuf::from))
        .unwrap_or_else(|| {
            eprintln!("Error: --file-dir is required (or set in config file at ~/.config/cp_unfold/config.toml)");
            std::process::exit(1);
        });

    let library_path = args.library_path.clone()
        .or_else(|| config.library_path.as_ref().map(PathBuf::from));

    // CLI引数で設定が指定された場合も保存（初回のみ）
    if !config_exists && (args.file_dir.is_some() || args.library_name.is_some() || args.library_path.is_some()) {
        let new_config = Config {
            file_dir: args.file_dir.as_ref().map(|p| p.display().to_string()),
            library_name: args.library_name.clone(),
            library_path: args.library_path.as_ref().map(|p| p.display().to_string()),
        };
        if let Err(e) = new_config.save() {
            eprintln!("Warning: Could not save config file: {}", e);
        } else {
            eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
        }
    }

    let mut unfold = Unfold::from_args(
        src,
        library_name,
        file_dir,
        library_path,
    );
    let res = unfold.unfold();

    match res {
        Ok(val) => {
            println!("{}", val);
        },
        Err(val) => {
            eprintln!("{:?}", val);
            std::process::exit(1);
        }
    }

}
