use std::fs;
use std::path::{Path, PathBuf};

pub fn run(files: &[String], cache_dir: &Path) {
    if cache_dir.exists() {
        fs::remove_dir_all(cache_dir).unwrap();
    }
    fs::create_dir_all(cache_dir).unwrap();

    let paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();

    let options = fs_extra::dir::CopyOptions::new();

    match fs_extra::copy_items(&paths, cache_dir, &options) {
        Ok(_) => println!("✨ Shelved {} items.", files.len()),
        Err(e) => eprintln!("❌ Error shelving files: {}", e),
    }
}
