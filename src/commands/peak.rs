use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn peak(cache_dir: &Path) {
    if !cache_dir.exists() {
        println!("{}", format!("Shelf is empty.").red());
        return;
    }

    let entries = match fs::read_dir(cache_dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("{}", "Error: ❌ Could not read shelf contents.".red());
            return;
        }
    };

    println!(
        "{}",
        format!("Currently shelved items:").bright_white().bold()
    );
    let mut count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() {
            println!("  {}", name.bright_white().bold());
        } else {
            println!("  {}", name.bright_white());
        }
        count += 1;
    }

    if count == 0 {
        println!("{}", format!("Shelf is empty.").red());
    }
}
