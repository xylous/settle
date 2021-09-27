use std::fs::{read_to_string, write};
use std::path::PathBuf;
use glob::glob;

pub fn file_to_string(path: &str) -> String
{
    read_to_string(path).expect("failed to read file")
}

pub fn write_to_file(path: &str, data: &str)
{
    write(path, data).expect("Unable to write file")
}

pub fn replace_extension(file: &str, new_ext: &str) -> String
{
    let mut path = PathBuf::from(file);
    path.set_extension(new_ext);
    path.to_string_lossy().to_string()
}

pub fn list_md_files() -> Vec<String>
{
    glob("*.md")
        .expect("failed to read directory")
        .map(|f| f.unwrap().to_string_lossy().to_string())
        .collect()
}
