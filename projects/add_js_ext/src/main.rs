use anyhow::Result;
use glob::glob;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[91mError: {}\x1b[0m", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("\x1b[93mUsage: aje <INPUT pattern>\x1b[0m");
        std::process::exit(1);
    }
    let pattern = &args[1];
    let (count_processed, _count_total) = process_files(pattern)?;

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

fn process_files(pattern: &str) -> Result<(usize, usize)> {
    let mut count_processed = 0;
    let mut count_total = 0;

    for entry in glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            count_total += 1;
            if process_file(&path, &mut count_processed)? {
                // Count is incremented within process_file if the file is modified
            }
        }
    }

    Ok((count_processed, count_total))
}

fn process_file(file_path: &Path, count: &mut usize) -> Result<bool> {
    let content = fs::read_to_string(file_path)?;
    let modified_content = modify_paths(&content, file_path.parent().unwrap())?;

    if content != modified_content {
        fs::write(file_path, modified_content)?;
        println!("\x1b[96mProcessed: {}\x1b[0m", file_path.display());
        *count += 1;
        Ok(true)
    } else {
        Ok(false)
    }
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
