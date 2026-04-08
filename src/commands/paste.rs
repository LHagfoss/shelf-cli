use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn paste(destination: Option<&String>, cache_dir: &Path) {
    let dest_str = destination.map(|s| s.as_str()).unwrap_or(".");
    let dest_path = Path::new(dest_str);

    if !dest_path.exists() {
        fs::create_dir_all(dest_path).unwrap();
    }

    if !cache_dir.exists() {
        eprintln!("{}", format!("Shelf is empty.").white().bold());
        return;
    }

    let entries = fs::read_dir(cache_dir).unwrap();
    let mut paths = Vec::new();
    for entry in entries {
        paths.push(entry.unwrap().path());
    }

    let options = fs_extra::dir::CopyOptions::new();
    match fs_extra::copy_items(&paths, dest_path, &options) {
        Ok(_) => println!(
            "{}",
            format!("Pasted {} items to '{}'", paths.len(), dest_str)
                .green()
                .bold()
        ),
        Err(e) => eprintln!("{}", format!("❌ Error pasting files: {}", e).red().bold()),
    }
}
