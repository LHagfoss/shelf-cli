use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn add(files: &[String], cache_dir: &Path) {
    if !cache_dir.exists() {
        if let Err(e) = fs::create_dir_all(cache_dir) {
            eprintln!("❌ {}", format!("Error creating shelf: {}", e).red().bold());
            return;
        }
    }

    let paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
    let options = fs_extra::dir::CopyOptions::new();

    match fs_extra::copy_items(&paths, cache_dir, &options) {
        Ok(_) => println!(
            "{}",
            format!("➕ Added {} items to shelf.", files.len())
                .green()
                .bold()
        ),
        Err(e) => eprintln!("❌ {}", format!("Error adding files: {}", e).red().bold()),
    }
}
