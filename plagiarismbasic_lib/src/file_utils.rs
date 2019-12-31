use std::path::PathBuf;
use std::{fs, io};

/// Returns a vector of tuples of (file name, file contents as string)
///     for each file in the directory.
///     Panics on error if it's a filename error
///     Silently replaces file contents by error string otherwise.
pub fn get_file_contents_from_dir(dir: &str) -> io::Result<Vec<(String, String)>> {
    let filepaths = get_file_paths_from_dir(dir)?;
    let mut file_id_contents: Vec<(String, String)> = Vec::new();
    for filepath in filepaths {
        let file_path_str = filepath
            .file_name()
            .expect("File name could not be converted from an OsStr to a String")
            .to_str()
            .unwrap()
            .to_string();
        file_id_contents.push((
            file_path_str.clone(),
            fs::read_to_string(filepath).expect(&format!(
                "{} cannot be read as an UTF-8 file! Please ensure it is in the UTF-8 format.",
                file_path_str
            )),
        ))
    }
    Ok(file_id_contents)
}

/// Returns a list of paths to files (not subdirectories) that are in a directory
fn get_file_paths_from_dir(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            paths.push(path);
        }
    }
    Ok(paths)
}
