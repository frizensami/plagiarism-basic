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
        println!(
            "Checking filename {:?}",
            filepath
                .file_name()
                .expect("File name could not be converted from an OsStr to a String")
        );
        file_id_contents.push((
            filepath.file_name().unwrap().to_str().unwrap().to_string(),
            fs::read_to_string(filepath)
                .expect("This text file cannot be read as UTF-8! Please convert it to UTF-8."),
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
