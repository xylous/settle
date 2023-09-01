use glob::glob;
use std::fs::{canonicalize, read_to_string, write};
use std::path::{Path, PathBuf};

/// Return true if the path specified exists and is a file
pub fn file_exists(path: &str) -> bool
{
    Path::new(path).is_file()
}

/// Return true if the path specified exists and is a dir
pub fn dir_exists(path: &str) -> bool
{
    Path::new(path).is_dir()
}

/// Return the last segment of a path
pub fn basename(path: &str) -> String
{
    let pieces: Vec<&str> = path.split('/').collect();
    pieces[pieces.len() - 1].to_string()
}

/// Return the first segments of a path
pub fn dirname(path: &str) -> String
{
    let pieces: Vec<&str> = path.split('/').collect();
    pieces[0..pieces.len() - 1].join("/")
}

/// Given a path, replace its extension with `new_ext` and return the resulting path
pub fn replace_extension(file: &str, new_ext: &str) -> String
{
    let mut path = PathBuf::from(file);
    path.set_extension(new_ext);
    path.to_string_lossy().to_string()
}

/// Read `path` and return the contents
pub fn file_to_string(path: &str) -> String
{
    read_to_string(path).unwrap_or_else(|_| panic!("Can't read file '{}'", path))
}

/// Write `data` to `path`
pub fn write_to_file(path: &str, data: &str)
{
    write(path, data).unwrap_or_else(|_| panic!("Can't write to file '{}'", path))
}

/// Rename `from` to `to`
pub fn rename(from: &str, to: &str)
{
    std::fs::rename(from, to).unwrap_or_else(|_| panic!("Can't rename file '{}' to '{}'", from, to))
}

/// Create specified `path` as a directory
pub fn mkdir(path: &str)
{
    std::fs::create_dir_all(path).unwrap_or_else(|_| panic!("Can't create directory '{}'", path))
}

/// List all markdown files in the specified directory
pub fn list_md_files(dir: &str) -> Vec<String>
{
    glob(&format!("{}/*.md", dir))
        .unwrap_or_else(|_| panic!("Can't read directory '{}'", dir))
        .map(|f| f.unwrap().to_string_lossy().to_string())
        .collect()
}

/// List all subdirectories in the specified directory
pub fn list_subdirectories(dir: &str) -> Vec<String>
{
    glob(&format!("{}/*/", dir))
        .unwrap_or_else(|_| panic!("Can't read directory '{}'", dir))
        .map(|f| f.unwrap().to_string_lossy().to_string())
        .collect()
}

/// Given a relative path, return an absolute path, or nothing (empty string) if it failed
pub fn abs_path(path: &str) -> String
{
    canonicalize(path)
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
