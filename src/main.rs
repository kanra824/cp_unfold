mod unfold;
use unfold::Unfold;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cp_unfold")]
#[command(about = "Unfold Rust source files by expanding imports", long_about = None)]
struct Args {
    /// Source file name to unfold
    #[arg(short, long, default_value = "main.rs")]
    src: String,

    /// Library import name (e.g., "library")
    #[arg(short, long, default_value = "library")]
    library_name: String,

    /// Directory containing the source file
    #[arg(short, long)]
    file_dir: PathBuf,

    /// Path to the library directory
    #[arg(short = 'p', long)]
    library_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut unfold = Unfold::from_args(
        args.src,
        args.library_name,
        args.file_dir,
        args.library_path,
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
