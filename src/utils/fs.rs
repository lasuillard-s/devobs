use std::{fs::create_dir_all,
          path::{Path, PathBuf}};

use anyhow::Result;
use glob::glob;

/// Create the file if it does not exist, including its parent directories.
pub(crate) fn touch_file(path: &Path) -> Result<()> {
    if path.exists() {
        log::debug!("File already exists: {}", path.display());
        return Ok(());
    }

    create_dir_all(
        path.parent()
            .expect("Failed to get parent directory for file creation."),
    )?;
    std::fs::File::create(path)?;
    log::debug!("Created file: {}", path.display());

    Ok(())
}

/// List files in the `from` directory based on the include and exclude patterns.
pub(crate) fn list_files(from: &Path, include: &[String], exclude: &[String]) -> Vec<PathBuf> {
    let mut include = expand_glob(from, include);
    let exclude = expand_glob(from, exclude);

    // Filter out files that match the exclude patterns
    include.retain(|path| {
        // Exclude files that match any of the exclude patterns
        !exclude.iter().any(|ex| path == ex)
    });

    include
}

/// Expand glob patterns in the given directory, returning a flat list of paths.
pub(crate) fn expand_glob(from: &Path, patterns: &[String]) -> Vec<PathBuf> {
    patterns
        .iter()
        .flat_map(|s| {
            glob(
                from.join(s)
                    .to_str()
                    .expect("Failed to convert path to string"),
            )
            .expect("Failed to create glob pattern")
        })
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    // TODO(lasuillard): Write unit tests
    #[test]
    fn test_nothing() {
        assert_eq!(1 + 1, 2);
    }
}
