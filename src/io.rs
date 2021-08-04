#![allow(dead_code)]
use std::fs::{self, File, read_to_string, write};
use std::path::{Path, PathBuf};

pub fn file_to_string(path: &str) -> String
{
    read_to_string(path).expect("failed to read file")
}

pub fn create_file(path: &str) -> File
{
    File::create(path).expect("failed to create path")
}

pub fn write_to_file(path: &str, data: &str)
{
    write(path, data).expect("Unable to write file")
}

pub fn path_exists(path: &str) -> bool
{
    Path::new(path).exists()
}

pub fn replace_extension(file: &str, new_ext: &str) -> String
{
    let mut path = PathBuf::from(file);
    path.set_extension(new_ext);
    path.to_string_lossy().to_string()
}

pub fn remove_file(path: &str) -> std::io::Result<()>
{
    fs::remove_file(path)
}
