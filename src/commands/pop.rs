use crate::commands::clear::clear;
use crate::commands::paste::paste;
use std::path::Path;

pub fn pop(destination: Option<&String>, cache_dir: &Path) {
    paste(destination, cache_dir);
    // After a successful paste, we clear the shelf
    clear(cache_dir);
}
