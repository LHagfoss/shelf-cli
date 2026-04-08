use colored::*;
use std::fs;
use std::path::Path;

pub fn clear(cache_dir: &Path) {
    if cache_dir.exists() {
        if let Err(e) = fs::remove_dir_all(cache_dir) {
            eprintln!("{}", format!("Failed to clear shelf: {}", e).red());
            return;
        }
    }
    println!("{}", format!("Shelf cleared!").bold());
}
