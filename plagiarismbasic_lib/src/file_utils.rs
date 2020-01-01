use std::fs;
use std::path::PathBuf;

/// Returns a vector of tuples of (file name, file contents as string)
///     for each file in the directory.
///     Panics on error if it's a filename error
///     Silently replaces file contents by error string otherwise.
pub fn get_file_contents_from_dir(dir: &str) -> Vec<(String, String)> {
    let filepaths = get_file_paths_from_dir(dir);
    let mut file_id_contents: Vec<(String, String)> = Vec::new();
    for filepath in filepaths {
        let file_path_str = filepath
            .file_name()
            .expect("Invalid file path! (likely terminating in /..)")
            .to_str()
            .expect("File name could not be converted from an OsStr to a String!")
            .to_string();
        file_id_contents.push((
            file_path_str.clone(),
            fs::read_to_string(filepath).unwrap_or_else(|_| {
                panic!(
                    "{} cannot be read as an UTF-8 file! Please ensure it is in the UTF-8 format.",
                    file_path_str
                )
            }),
        ))
    }
    file_id_contents
}

/// Returns a list of paths to files (not subdirectories) that are in a directory
fn get_file_paths_from_dir(dir: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap_or_else(|_| {
        panic!(
            "Path {} doesn't exist, no permissions, or is not a directory!",
            dir
        )
    }) {
        let path = entry.expect("Cannot parse this directory entry!").path();
        if !path.is_dir() {
            paths.push(path);
        }
    }
    paths
}
