use anyhow::Result;
use glob::glob;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: aje <INPUT pattern>");
        std::process::exit(1);
    }
    let pattern = &args[1];
    let files = find_files(pattern)?;

    for file in files {
        process_file(&file)?;
    }

    Ok(())
}

fn find_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn process_file(file_path: &Path) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let modified_content = modify_paths(&content, file_path.parent().unwrap())?;

    if content != modified_content {
        fs::write(file_path, modified_content)?;
        println!("Processed: {}", file_path.display());
    }

    Ok(())
}

fn modify_paths(content: &str, directory: &Path) -> Result<String> {
    let re_import = Regex::new(r#"import\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#)?;
    let re_export = Regex::new(r#"export\s+([\w\s{},*]+)\s+from\s+['"](.+?)['"]"#)?;
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
