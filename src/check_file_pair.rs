use crate::GlobalOpts;
use anyhow::{Result, anyhow, bail};
use clap::Args;
use glob::glob;
use regex::{self, Regex};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::{PathBuf, absolute};
use strfmt::strfmt;

/// Check for matching file exists.
#[derive(Args, Debug, Clone)]
pub struct CommandArgs {
    /// Directory to check for matching files.
    #[arg(long)]
    from: String,

    /// Directory where the expected files should be located.
    #[arg(long)]
    to: String,

    /// List of glob patterns to include files from the `from` directory.
    ///
    /// This option can be specified multiple times or as a comma-separated list.
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    include: Vec<String>,

    /// List of glob patterns to exclude files from the `from` directory.
    ///
    /// This option can be specified multiple times or as a comma-separated list.
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    exclude: Vec<String>,

    /// Expected pattern for the file in the `to` directory.
    ///
    /// Variables available for substitution:
    ///
    /// - `{cwd}`: current working directory
    ///
    /// - `{from}`: absolute path to the `from` directory
    ///
    /// - `{to}`: absolute path to the `to` directory
    ///
    /// - `{filename}`: full filename (including extension)
    ///
    /// - `{stem}`: file stem (name without extension)
    ///
    /// - `{extension}`: file extension
    ///
    /// - `{relative_from}`: relative path from the `from` directory to the file
    ///
    #[arg(long, default_value = "{to}/{relative_from}/{filename}")]
    expect: String,

    /// Regex to apply to the `filename` variable. Capture groups will then be available
    /// as variables in the `expect` string, overriding the default values.
    #[arg(long)]
    filename_regex: Option<Regex>,

    /// If the expected file does not exist, create it.
    #[arg(long)]
    create_if_not_exists: bool,
}

pub fn command(args: CommandArgs, global_opts: GlobalOpts) -> Result<()> {
    let mut missing_files = vec![] as Vec<PathBuf>;

    // Preprocess options
    let from = absolute(PathBuf::from(&args.from))?;
    let to = absolute(PathBuf::from(&args.to))?;
    let cwd = current_dir()?;

    // Prepare base variables for substitution
    let mut base_vars = HashMap::new();
    base_vars.insert("cwd".to_string(), cwd.to_str().unwrap());
    base_vars.insert("from".to_string(), from.to_str().unwrap());
    base_vars.insert("to".to_string(), to.to_str().unwrap());
    log::debug!("Prepared base variables: {:?}", base_vars);

    for path in list_files(&from, &args.include, &args.exclude) {
        log::trace!("Checking file {}", path.display());

        let mut vars = base_vars.clone();
        let filename = path.file_name().expect("Failed to get file name");
        let stem = path
            .file_stem()
            .ok_or(anyhow!("Failed to get file stem"))?
            .to_str()
            .ok_or(anyhow!("Failed to convert file stem to string"))?;

        let extension = path
            .extension()
            .ok_or(anyhow!("Failed to get file extension"))?
            .to_str()
            .ok_or(anyhow!("Failed to convert file extension to string"))?;

        let relative_from = path
            .strip_prefix(&from)?
            .parent()
            .ok_or(anyhow!("Failed to get parent directory"))?
            .to_str()
            .ok_or(anyhow!("Failed to convert parent directory to string"))?;

        // If empty string, use "."
        let relative_from = if relative_from.is_empty() {
            ".".to_string()
        } else {
            relative_from.to_string()
        };

        // Prepare variables for substitution
        vars.insert("stem".to_string(), stem);
        vars.insert("extension".to_string(), extension);
        vars.insert("relative_from".to_string(), &relative_from);
        vars.insert("filename".to_string(), filename.to_str().unwrap());
        if let Some(ref regex) = args.filename_regex {
            let captures = regex.captures(filename.to_str().unwrap()).unwrap();
            for (name, value) in regex
                .capture_names()
                .flatten()
                .filter_map(|n| Some((n, captures.name(n)?.as_str())))
            {
                vars.insert(name.to_string(), value);
            }
        }
        log::debug!(
            "Prepared substitution variables for file {:?}: {:?}",
            path,
            vars
        );

        // Render the expected file path
        let result = strfmt(&args.expect, &vars)?;
        log::trace!("Formatted result: {}", result);

        let result_path = absolute(PathBuf::from(&result))?;
        log::trace!("Resolved result path: {}", result_path.display());

        // Check if the expected file exists
        if result_path.exists() {
            log::debug!("Expected file exists: {}", result_path.display());
            continue;
        }

        log::warn!(
            "Pair of file {} does not exist: {}",
            path.display(),
            result_path.display(),
        );
        missing_files.push(result_path);
    }

    // Check missing files and create if requested
    if !missing_files.is_empty() {
        if !args.create_if_not_exists {
            bail!(
                "There are {} missing files. Use `--create-if-not-exists` to create them.",
                missing_files.len()
            );
        }
        for missing in &missing_files {
            log::warn!("Creating missing file: {}", missing.display());
            if !global_opts.dry_run {
                touch_file(&missing)?;
            }
        }
        bail!("Created {} missing files.", missing_files.len());
    }

    log::info!("Everything is fine, no missing files.");
    Ok(())
}

/// Create the file if it does not exist, including its parent directories.
fn touch_file(path: &PathBuf) -> Result<()> {
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
fn list_files(from: &PathBuf, include: &Vec<String>, exclude: &Vec<String>) -> Vec<PathBuf> {
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
fn expand_glob(from: &PathBuf, patterns: &Vec<String>) -> Vec<PathBuf> {
    patterns
        .iter()
        .map(|s| {
            glob(
                from.join(s)
                    .to_str()
                    .expect("Failed to convert path to string"),
            )
            .expect("Failed to create glob pattern")
        })
        .flatten()
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
