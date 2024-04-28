use std::path::PathBuf;

use clap::Parser;
use readir::{dir::DirEntry, Result};

#[derive(Debug, Parser)]
#[command(name = "readir", version, about = "List all files in a directory")]
struct CliArgs {
    #[clap(short, long)]
    directory: PathBuf,

    #[clap(short, long, default_value = "false")]
    all: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let dir = args.directory;
    let display_all = args.all;

    if !dir.is_dir() {
        eprintln!("Error: provided is not a directory");
        std::process::exit(1);
    }

    let entry = DirEntry::new(dir.clone())?;
    if args.all {
        println!(
            "Listing all files in directory and subdirectories: {}",
            dir.display()
        );
        println!("{}", entry);
    } else {
        println!("Listing files in directory: {}", dir.display());
        for file in entry.list_files_in_root(display_all) {
            match file.strip_prefix(&dir) {
                Ok(relative_path) => println!("{}", relative_path.display()),
                Err(_) => println!("{}", file.display()),
            }
        }
    }

    Ok(())
}
