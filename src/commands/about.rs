use colored::*;

pub fn about() {
    println!("{}", format!("Shelf CLI. \n").bold());

    println!("{}", format!("Version: {}", env!("CARGO_PKG_VERSION")));
    println!("A \"git stash\" like cli, for files and folders.\n");

    println!(
        "My thoughts about making this was mainly because there was nothing like it out there."
    );
    println!("And at the moment while creating this I am learning the Rust language.");
    println!("So why not create something useful for myself with it?\n");

    println!("Made by me");
    println!("{}", format!("https://lhagfoss.com").blue());
}
