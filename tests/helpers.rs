#![allow(dead_code)]
use std::{collections::HashMap, fs::create_dir_all, path::Path, process::Output};

use assert_cmd::Command;
use tempfile::{TempDir, tempdir};

#[macro_export]
macro_rules! to_str {
    ($value:expr) => {
        $value.to_str().expect("Failed to convert to string")
    };
}

/// Create a command for the current binary being tested.
pub(crate) fn get_cmd() -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    cmd.arg("--no-colors");
    cmd.env("RUST_BACKTRACE", "0");
    cmd
}

/// Normalize console output by replacing dynamic content (e.g. temp dir) with static placeholders.
pub(crate) fn normalize_console_output<S: AsRef<str>, T: ToString + std::fmt::Display>(
    output: S,
    replace: HashMap<T, &str>,
) -> String {
    let mut output = output.as_ref().trim().to_string();

    // Replace all occurrences of keys in the output with their corresponding values
    // This is useful for normalizing paths or other dynamic content in the output
    for (key, value) in replace {
        output = output.replace(&key.to_string(), value);
    }

    output
}

/// Creates a temporary directory files populated.
///
/// The `files` parameter is a map where the key is the file path (relative to the temp directory)
/// and the value representing the file content.
pub(crate) fn get_temp_dir(files: HashMap<&str, &str>) -> TempDir {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let dir_path = temp_dir.path();
    for (path, content) in files {
        let full_path = dir_path.join(&path);

        // If path ends with a slash, create a directory and continue
        if path.ends_with("/") {
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

        // Write the file with the specified content
        std::fs::write(full_path, content).expect("Failed to write file");
    }

    // NOTE: Return the `TempDir` to ensure it lives long enough; it will be deleted when dropped
    temp_dir
}

/// Helper function to list all files in a directory recursively,
/// excluding `.git` directories and returning relative paths.
pub(crate) fn list_dir(dir: &Path) -> Vec<String> {
    glob::glob(to_str!(dir.join("**/*")))
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
