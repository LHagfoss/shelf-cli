use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn drop(item_name: &str, cache_dir: &Path) {
    let target = cache_dir.join(item_name);

    if !target.exists() {
        eprintln!(
            "❌ {}",
            format!("Item '{}' not found in shelf.", item_name)
                .red()
                .bold()
        );
        return;
    }

    let result = if target.is_dir() {
        fs::remove_dir_all(&target)
    } else {
        fs::remove_file(&target)
    };

    match result {
        Ok(_) => println!(
            "{}",
            format!("🗑️  Dropped '{}' from shelf.", item_name)
                .yellow()
                .bold()
        ),
        Err(e) => eprintln!(
            "❌ {}",
            format!("Error dropping item '{}': {}", item_name, e)
                .red()
                .bold()
        ),
    }
}
