use std::fs::{File, read_to_string, write};
use std::path::Path;

pub fn file_to_string(path: &str) -> String
{
    read_to_string(path).expect("failed to read file")
}

pub fn create_file(path: &str) -> File
{
    File::create(path).expect("failed to create path")
}

pub fn write_to_file(path: &str, data: &str) -> ()
{
    write(path, data).expect("Unable to write file")
}

pub fn path_exists(path: &str) -> bool
{
    Path::new(path).exists()
}
