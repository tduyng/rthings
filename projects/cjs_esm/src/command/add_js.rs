use std::{fs, path::Path};

use crate::error::CEsmError;
use clap::Parser;
use glob::glob;
use regex::Regex;

/// Arguments for the `addjs` command
#[derive(Parser)]
pub struct AddJsArgs {
    /// The glob pattern to match TypeScript files
    #[arg()]
    pub input: String,
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
pub fn run_add_js(pattern: &str) -> Result<(), CEsmError> {
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

fn process_files(pattern: &str) -> Result<usize, CEsmError> {
    let mut count_processed = 0;

    for entry in glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            process_file(&path, &mut count_processed)?;
        }
    }

    Ok(count_processed)
}

fn process_file(file_path: &Path, count: &mut usize) -> Result<(), CEsmError> {
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
fn modify_paths(content: &str, directory: &Path) -> Result<String, CEsmError> {
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
