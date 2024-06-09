//! # cesm

//! `cesm` is a command-line tool designed to process TypeScript files by adding `.js` to the import and export paths. This can be useful when migrate a TypeScript projects from Commonjs to ESM.

//! ## Features

//! - **Add `.js` to import/export paths**: Automatically modifies TypeScript files to include `.js` extensions in import and export statements.
//! - **Glob pattern matching**: Specify patterns to match files using glob syntax.
//! - **User-friendly CLI**: Intuitive and easy-to-use command-line interface powered by `clap`.

//! ## Installation

//! - To install `cesm`, you need to have Rust and Cargo installed. Then you can build the project from the source:

//! ```sh
//! git clone https://github.com/tduyng/rthings.git
//! cd rthings/projects/cjs_esm
//! cargo install --path .
//! ```
//! - You can also install directly from the GitHub repository using the following command:

//! ```sh
//! git install --git https://github.com/tduyng/rthings/tree/main/projects/cjs_esm
//! ```

//! ## Usage

//! ### Commands

//! #### `addjs`

//! This command processes TypeScript files to add `.js` to import/export paths.

//! ```sh
//! cesm addjs <PATTERN>
//! ```

//! - **PATTERN**: The glob pattern to match TypeScript files.

//! ### Examples

//! Process all TypeScript files in the `src` directory and its subdirectories:

//! ```sh
//! cesm addjs "src/**/*.ts"
//! ```

//! Process a specific file:

//! ```sh
//! cesm addjs "src/main.ts"
//! ```

//! ### Notes

//! - If no files matching the pattern were modified, a message indicating this will be printed.
//! - If files are modified, the count of processed files will be printed.

//! ## Development

//! ### Dependencies

//! - **[clap](https://github.com/clap-rs/clap)**: For command-line argument parsing.
//! - **[anyhow](https://github.com/dtolnay/anyhow)**: For error handling.
//! - **[glob](https://github.com/rust-lang-nursery/glob)**: For file pattern matching.
//! - **[regex](https://github.com/rust-lang/regex)**: For regular expression processing.

//! ### Running Locally

//! To run the project locally for development purposes:

//! ```sh
//! cargo run -- addjs "src/**/*.ts"
//! ```
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
use anyhow::Result;
use clap::{Parser, Subcommand};
use glob::glob;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

/// A CLI program for adding .js to import/export paths in TypeScript projects
#[derive(Parser)]
#[command(
    name = "cesm",
    version = env!("CARGO_PKG_VERSION"),
)]
#[doc(hidden)]
struct Args {
    /// Subcommands to execute specific tasks
    #[command(subcommand)]
    cmd: Option<Command>,
}

/// Enumeration of available subcommands
#[derive(Subcommand)]
#[doc(hidden)]
enum Command {
    /// Add .js to import/export paths in TypeScript projects
    #[command(name = "addjs")]
    AddJs(AddJsArgs),
}

/// Arguments for the `addjs` command
#[derive(Parser)]
#[doc(hidden)]
struct AddJsArgs {
    /// The glob pattern to match TypeScript files
    #[arg()]
    input: String,
}

#[doc(hidden)]
fn main() {
    let args = Args::parse();
    match args.cmd {
        Some(Command::AddJs(run_args)) => {
            if let Err(err) = run_add_js(&run_args.input) {
                eprintln!("\x1b[91mError: {}\x1b[0m", err);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("\x1b[91mCommand not found\x1b[0m");
        }
    }
}

/// Runs the main processing logic for the `addjs` command
///
/// # Arguments
///
/// * `pattern` - The glob pattern to match TypeScript files
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if successful, otherwise an error
#[doc(hidden)]
fn run_add_js(pattern: &str) -> Result<()> {
    let count_processed = process_files(pattern)?;

    if count_processed == 0 {
        println!("\x1b[93mNo files were modified.\x1b[0m");
    } else if count_processed == 1 {
        println!("\x1b[92mProcessed 1 file successfully\x1b[0m",);
    } else {
        println!(
            "\x1b[92mProcessed {} files successfully\x1b[0m",
            count_processed
        );
    }

    Ok(())
}

/// Processes files matching the given glob pattern
///
/// # Arguments
///
/// * `pattern` - The glob pattern to match TypeScript files
///
/// # Returns
///
/// * `Result<usize>` - The number of files processed successfully
#[doc(hidden)]
fn process_files(pattern: &str) -> Result<usize> {
    let mut count_processed = 0;

    for entry in glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            process_file(&path, &mut count_processed)?;
        }
    }

    Ok(count_processed)
}

/// Processes a single file, modifying import/export paths if necessary
///
/// # Arguments
///
/// * `file_path` - The path to the file to process
/// * `count` - A mutable reference to the count of processed files
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if successful, otherwise an error
#[doc(hidden)]
fn process_file(file_path: &Path, count: &mut usize) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let modified_content = modify_paths(&content, file_path.parent().unwrap())?;

    if content != modified_content {
        fs::write(file_path, modified_content)?;
        println!("\x1b[96mProcessed: {}\x1b[0m", file_path.display());
        *count += 1;
    }

    Ok(())
}

/// Modifies import/export paths in the file content
///
/// This function scans the provided file content for import, export, and dynamic import statements
/// and updates their paths to include ".js" if necessary, based on the files present in the specified `directory`.
///
/// # Arguments
///
/// * `content` - The content of the file as a string slice
/// * `directory` - The directory of the file as a `Path` reference
///
/// # Returns
///
/// * `Result<String>` - The modified file content with updated import/export paths
///
/// # Regex Patterns
///
/// 1. **Import Statements**: Matches TypeScript/JavaScript import statements
///    - Pattern: `import { Component } from './component'`
///    - Regex: `r#"import\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#`
///
/// 2. **Export Statements**: Matches TypeScript/JavaScript export statements
///    - Pattern: `export { Component } from './component'`
///    - Regex: `r#"export\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#`
///
/// 3. **Dynamic Import Statements**: Matches dynamic import statements in JavaScript/TypeScript
///    - Pattern: `await import('./component')`
///    - Regex: `r#"import\(['"](.+?)['"]\)"#`
///
/// # Example
///
/// ```rust
/// let content = r#"import { Component } from './component';
/// export { Component } from './component';
/// await import('./component');"#;
/// let directory = Path::new("/path/to/your/ts/files");
/// let modified_content = modify_paths(content, &directory)?;
/// println!("{}", modified_content);
/// ```
#[doc(hidden)]
fn modify_paths(content: &str, directory: &Path) -> Result<String> {
    // Regex to match import statements
    // Example: import { Component } from './component'
    let re_import = Regex::new(r#"import\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#)?;

    // Regex to match export statements
    // Example: export { Component } from './component'
    let re_export = Regex::new(r#"export\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#)?;

    // Regex to match dynamic import statements
    // Example: await import('./component')
    let re_dynamic_import = Regex::new(r#"import\(['"](.+?)['"]\)"#)?;

    let modified_content_import = re_import
        .replace_all(content, |caps: &regex::Captures| {
            let imports = &caps[1];
            let original_path = &caps[2];
            let modified_path = modify_path(original_path, directory);
            format!("import {} from '{}'", imports, modified_path)
        })
        .into_owned();

    let modified_content_export = re_export
        .replace_all(&modified_content_import, |caps: &regex::Captures| {
            let exports = &caps[1];
            let original_path = &caps[2];
            let modified_path = modify_path(original_path, directory);
            format!("export {} from '{}'", exports, modified_path)
        })
        .into_owned();

    let modified_content_dynamic_import = re_dynamic_import
        .replace_all(&modified_content_export, |caps: &regex::Captures| {
            let original_path = &caps[1];
            let modified_path = modify_path(original_path, directory);
            format!("import('{}')", modified_path)
        })
        .into_owned();

    Ok(modified_content_dynamic_import)
}

/// Modifies the import/export path to include .js if necessary
///
/// This function checks if the given `original_path` needs to have ".js" appended
/// based on the presence of corresponding TypeScript or JavaScript files in the specified `directory`.
/// It prioritizes "index" files and various file extensions in a specific order.
///
/// # Arguments
///
/// * `original_path` - The original import/export path as a string slice
/// * `directory` - The directory of the file as a `Path` reference
///
/// # Returns
///
/// * `String` - The modified import/export path with ".js" appended if necessary
///
/// # Algorithm
///
/// 1. First, it checks for "index" files with common TypeScript and JavaScript extensions.
///    If an "index" file exists (e.g., "index.ts", "index.js"), it returns the path with "/index.js" appended.
/// 2. If no "index" file is found, it checks for regular files with the same base name but different extensions.
///    If a matching file is found (e.g., "module.ts", "module.js"), it returns the path with ".js" appended.
/// 3. If no matching files are found, it returns the original path unchanged.
///
/// # Example
///
/// ```rust
/// let directory = Path::new("/path/to/your/ts/files");
/// let original_path = "module";
/// let modified_path = modify_path(original_path, &directory);
/// println!("{}", modified_path); // Outputs: "module.js" or "module/index.js" if corresponding files exist
/// ```
#[doc(hidden)]
fn modify_path(original_path: &str, directory: &Path) -> String {
    let index_extensions = [
        "index.tsx",
        "index.ts",
        "index.js",
        "index.mjs",
        "index.cjs",
    ];
    for ext in &index_extensions {
        let index_path = directory.join(original_path).join(ext);
        if index_path.exists() {
            if ext.ends_with(".mjs") || ext.ends_with(".cjs") {
                return format!("{}/{}", original_path, ext);
            } else {
                return format!("{}/index.js", original_path);
            }
        }
    }

    let file_extensions = ["tsx", "ts", "js", "mjs", "cjs"];
    for ext in &file_extensions {
        let file_path = directory.join(format!("{}.{}", original_path, ext));
        if file_path.exists() {
            if ext.ends_with(".mjs") || ext.ends_with(".cjs") {
                return format!("{}.{}", original_path, ext);
            } else {
                return format!("{}.js", original_path);
            }
        }
    }

    original_path.to_string()
}
