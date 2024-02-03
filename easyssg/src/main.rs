use clap::Parser;
use libeasyssg::{Page, Site};
use std::{
    fs::{self},
    path::Path,
};
use walkdir::WalkDir;

/// A simple static site generator
#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"), // Automatically gets the package name
    version = env!("CARGO_PKG_VERSION"), // Automatically gets the package version
    author = env!("CARGO_PKG_AUTHORS"), // Automatically gets the package authors
    about = env!("CARGO_PKG_DESCRIPTION") // Automatically gets the package description
)]
struct Args {
    /// Sets the input directory to use
    #[clap(value_parser)]
    input: String,
}

fn process_directory(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let _site = Site::new();

    for entry in WalkDir::new(input_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if extension == "md" {
                let markdown = fs::read_to_string(path)?;
                let page = Page::new(&markdown);
                let html = page.get_html();

                let relative_path = path.strip_prefix(input_path)?.with_extension("html");
                let output_file_path = output_path.join(relative_path);

                if let Some(parent) = output_file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(output_file_path, html)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    let input_path = Path::new(&args.input);
    let build_path = input_path.join("_build");

    if build_path.exists() {
        eprintln!("Error: '_build' directory already exists in the specified path.");
        std::process::exit(1);
    } else {
        fs::create_dir(&build_path).expect("Failed to create '_build' directory");
    }

    println!("Generating site...");
    if let Err(e) = process_directory(input_path, &build_path) {
        eprintln!("Site generation failed: {e}");
        std::process::exit(1);
    }

    println!("Site generated successfully.");
}
