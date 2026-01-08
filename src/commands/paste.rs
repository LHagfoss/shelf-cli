use std::fs;
use std::path::Path;

pub fn run(destination: Option<&String>, cache_dir: &Path) {
    let dest_str = destination.as_deref().unwrap_or(".");
    let dest_path = Path::new(dest_str);

    if !dest_path.exists() {
        fs::create_dir_all(dest_path).unwrap();
    }

    if !cache_dir.exists() {
        eprintln!("📭 Shelf is empty.");
        return;
    }

    let entries = fs::read_dir(cache_dir).unwrap();
    let mut paths = Vec::new();
    for entry in entries {
        paths.push(entry.unwrap().path());
    }

    let options = fs_extra::dir::CopyOptions::new();
    match fs_extra::copy_items(&paths, dest_path, &options) {
        Ok(_) => println!("🚚 Pasted {} items to '{}'", paths.len(), dest_str),
        Err(e) => eprintln!("❌ Error pasting files: {}", e),
    }
}
