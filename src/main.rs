use anyhow::Result;
use clap::Parser;
use std::{fs, io::Write};

/// Recursively renames all css class names in all files with the specified file extension (css by default) in the root path and subdirectories.
#[derive(Parser)]
struct Cli {
    old_class: String,
    new_class: String,
    root_path: std::path::PathBuf,

    // Default extension is .css
    #[arg(short, long, default_value_t = String::from("css"))]
    file_extension: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!(
        "Change '{}' to '{}' in all files ending with .{} in directory '{}'\n",
        args.old_class,
        args.new_class,
        args.file_extension,
        args.root_path.to_string_lossy()
    );

    search_files(
        &args.old_class,
        &args.new_class,
        args.root_path,
        &args.file_extension,
    )
    .expect("Something went wrong");

    Ok(())
}

fn search_files(
    old_name: &str,
    new_name: &str,
    path: std::path::PathBuf,
    file_extension: &str,
) -> Result<()> {
    let paths = fs::read_dir(path)?;

    for path in paths {
        let tmp = path.expect("there is not path");

        if tmp.path().is_file() {
            rename_css_in_file(old_name, new_name, tmp, file_extension);
        } else {
            search_files(old_name, new_name, tmp.path(), file_extension)
                .expect("cloud not found subdirectory ");
        }
    }

    Ok(())
}

fn rename_css_in_file(
    old_name: &str,
    new_name: &str,
    file_path: fs::DirEntry,
    file_extension: &str,
) {
    let path = &file_path.path();

    match path.extension().and_then(std::ffi::OsStr::to_str) {
        None => return,
        Some(_file_extension) => {
            if _file_extension != file_extension {
                return;
            }
        }
    }

    println!("Update file: {:?}", path);

    let file_content = fs::read_to_string(path).expect("File not existing");
    let mut new_file_content = String::from("");

    for line in file_content.lines() {
        new_file_content = new_file_content + line.replace(old_name, new_name).as_str() + "\n";
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.to_str().unwrap())
        .unwrap();

    file.write(new_file_content.as_bytes())
        .expect("cloud not write to file");
}
