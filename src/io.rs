use glob::glob;
use std::fs::{canonicalize, read_to_string, write};
use std::path::{Path, PathBuf};

/// Read `path` and return the contents
pub fn file_to_string(path: &str) -> String
{
    read_to_string(path).expect("failed to read file")
}

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

/// Write `data` to `path`
pub fn write_to_file(path: &str, data: &str)
{
    write(path, data).expect("Unable to write file")
}

/// Rename `from` to `to`
pub fn rename(from: &str, to: &str)
{
    std::fs::rename(from, to).unwrap();
}

/// Given a filename, replace its extension with `new_ext`
pub fn replace_extension(file: &str, new_ext: &str) -> String
{
    let mut path = PathBuf::from(file);
    path.set_extension(new_ext);
    path.to_string_lossy().to_string()
}

/// List all markdown files in the specified directory
pub fn list_md_files(dir: &str) -> Vec<String>
{
    glob(&format!("{}/*.md", dir)).expect("failed to read directory")
                                  .map(|f| f.unwrap().to_string_lossy().to_string())
                                  .collect()
}

/// Create specified `path` as a directory
pub fn mkdir(path: &str)
{
    std::fs::create_dir_all(path).expect("Wasn't able to create directory:")
}

/// List all subdirectories in the specified directory
pub fn list_subdirectories(dir: &str) -> Vec<String>
{
    glob(&format!("{}/*/", dir)).expect("failed to read directory")
                                .map(|f| f.unwrap().to_string_lossy().to_string())
                                .collect()
}

/// Given a relative path, return an absolute path, or nothing (empty string) if it failed
pub fn abs_path(path: &str) -> String
{
    canonicalize(path).unwrap_or_default()
                      .to_string_lossy()
                      .to_string()
}
