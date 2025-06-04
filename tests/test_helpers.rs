#![allow(dead_code)]
use std::{collections::HashMap, fs::create_dir_all, path::Path, process::Output};

use tempfile::tempdir;

/// Creates a temporary Git repository with the specified files.
///
/// The `files` parameter is a map where the key is the file path (relative to the temp directory)
/// and the value representing the file content. If the value is `None`, the file
/// will be created empty.
pub(crate) fn temp_git_dir(files: Option<HashMap<&str, Option<&str>>>) -> tempfile::TempDir {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let dir_path = temp_dir.path();

    // Move to the temp directory and run initialize a new Git repository
    std::process::Command::new("git")
        .arg("init")
        .current_dir(dir_path)
        .status()
        .expect("Failed to execute git init");

    for (path, content) in files.unwrap_or_default() {
        let full_path = dir_path.join(&path);
        if full_path.ends_with("/") {
            // If the path ends with a slash, create a directory
            create_dir_all(&full_path).expect("Failed to create directory");
            continue;
        }

        // Ensure the parent directory exists
        create_dir_all(
            full_path
                .parent()
                .expect("Failed to get parent directory for file creation"),
        )
        .expect("Failed to create directory");

        // Write the file with the specified content or an empty string if None
        std::fs::write(full_path, content.unwrap_or_default()).expect("Failed to write file");
    }

    temp_dir // Return the temporary directory
}

/// Helper function to list all files in a directory recursively,
/// excluding `.git` directories and returning relative paths.
pub(crate) fn list_dir(dir: &Path) -> Vec<String> {
    glob::glob(dir.join("**/*").to_str().unwrap())
        .expect("Failed to create glob pattern")
        .filter_map(Result::ok)
        // Exclude .git directories
        .filter(|path| !path.components().any(|comp| comp.as_os_str() == ".git"))
        // Only include files
        .filter(|path| path.is_file())
        // Strip the directory prefix and convert to String
        .map(|path| {
            path.strip_prefix(dir)
                .expect("Failed to strip prefix")
                .to_str()
                .expect("Failed to convert path to str")
                .to_owned()
        })
        .collect()
}

/// Parses the output of a command execution, returning the standard output and error as strings.
// TODO(lasuillard): Add argument for normalization (such as temporary paths)
pub(crate) fn parse_output(output: &Output) -> (String, String) {
    let stdout = String::from_utf8_lossy(output.stdout.as_ref());
    let stderr = String::from_utf8_lossy(output.stderr.as_ref());
    (stdout.to_string(), stderr.to_string())
}

/// Returns the first line of a string, or an empty string if the input is empty.
pub(crate) fn first_line(s: String) -> String {
    s.lines().next().unwrap_or("").to_string()
}
