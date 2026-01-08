<div align="center">

# Shelf 🍱

**A temporary, disk-based holding buffer for files and folders in your terminal.**

<img src="image.png" alt="Shelf CLI Demo Screenshot" width="800" style="border-radius: 10px; margin: 20px 0;" />

</div>

## The Problem

Standard terminal clipboards (`pbcopy`, `xclip`) are great for text, but they choke on files and folders.

Sometimes you just need to grab a few files from deep inside one project, navigate to an entirely different part of your system, and drop them there. You don't want to construct a long `cp` command with complex absolute paths.

I couldn't find a simple tool that acted like a "holding area" for files across terminal sessions.

## The Solution: Shelf

Think of `shelf` as a **global, persistent `git stash` for your file system**.

It uses a temporary directory on your disk as a buffer. Because it uses the disk, it persists even if you close your terminal window or restart your computer, and it handles large files easily.

## Installation

Currently, the best way to install it is via Rust's `cargo`.

```bash
git clone [https://github.com/YOUR_USERNAME/shelf.git](https://github.com/YOUR_USERNAME/shelf.git)
cd shelf
cargo install --path .
```

Make sure your Cargo bin path (usually `~/.cargo/bin`) is in your shell's PATH.

## Usage

The workflow is simple: `copy` things onto the shelf, move somewhere else, and `paste` them off.

### 1. Copy (Shelve items)

This command clears whatever is currently on the shelf and copies the new items onto it.

```bash
# Copy specific files
shelf copy file1.tsx utils.ts

# Copy folders
shelf copy ./src/components ./assets/images
```

### 2. Paste (Unshelve items)

Copies everything currently on the shelf to the specified destination.

```bash
# Navigate somewhere else
cd ~/work/another-project

# Paste into the current directory
shelf paste

# OR paste into a specific folder
shelf paste ./src/legacy-code
```

## Help

The tool comes with built-in help thanks to Clap.

```bash
shelf --help
# or specific command help
shelf copy --help
```

---

Built with 🦀 Rust.
