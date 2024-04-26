use std::path::PathBuf;

use clap::Parser;
use readir::dir::DirEntry;

#[derive(Debug, Parser)]
#[command(name = "readir", version, about = "List all files in a directory")]
struct CliArgs {
    #[clap(short, long)]
    directory: PathBuf,
}

fn main() {
    let args = CliArgs::parse();
    let dir = args.directory;

    if !dir.is_dir() {
        eprintln!("Error: provided is not a directory");
        std::process::exit(1);
    }

    println!("Listing files in directory: {}", dir.display());
    let entries = DirEntry::entries(&dir, &dir);
    for entry in entries {
        println!("{}", entry)
    }
}
