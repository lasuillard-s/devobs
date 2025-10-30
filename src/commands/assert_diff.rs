use std::{fs::File,
          hash::{DefaultHasher, Hash, Hasher},
          io::Read,
          path::{PathBuf, absolute}};

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use serde::Serialize;

use crate::{GlobalOpts, utils::fs::list_files};

const BUFFER_SIZE: usize = 8192;

#[derive(ValueEnum, Clone, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
enum OnCommandError {
    /// Exit the program with the original error
    #[default]
    Propagate,

    /// Ignore the error and continue
    Ignore,
}

// NOTE: This command does not support dry-run mode, as there is no state change involved (except hash file).
/// Check for matching file exists.
#[derive(Args, Debug, Clone)]
pub(crate) struct CommandArgs {
    /// Target directory to watch for changes.
    #[arg(long)]
    target: String,

    /// List of glob patterns to include files from the `target` directory.
    ///
    /// This option can be specified multiple times or as a comma-separated list.
    #[arg(long, num_args = 1.., value_delimiter = ',', default_value = "**/*")]
    include: Vec<String>,

    /// List of glob patterns to exclude files from the `target` directory.
    ///
    /// This option can be specified multiple times or as a comma-separated list.
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    exclude: Vec<String>,

    /// Error handling strategy for the command.
    #[arg(long, default_value_t, value_enum)]
    on_command_error: OnCommandError,

    /// Command to run. First hash is computed before running the command, second hash after.
    /// If the hashes differ, an error is raised.
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

pub(crate) fn command(args: CommandArgs, _global_opts: GlobalOpts) -> Result<()> {
    // Prepare arguments
    let target = absolute(PathBuf::from(&args.target))?;
    if !target.exists() {
        bail!("Target path does not exist: {}", target.display());
    }

    // Calculate hash
    log::debug!("Calculating hash for: {}", target.display());
    let before_hash = calculate_directory_hash(&target, &args.include, &args.exclude)?;
    log::info!("Hash before command run: {}", before_hash);

    // Run command
    log::info!("Running command as child process: {:?}", args.command);
    let mut child = std::process::Command::new(&args.command[0])
        .args(&args.command[1..])
        .spawn()?;

    let status = child.wait()?;
    log::debug!("Command exited with status: {:?}", status);

    // Check for exit code
    if !status.success() {
        match args.on_command_error {
            OnCommandError::Ignore => {
                log::warn!(
                    "Command exited with non-zero status: {}, but ignoring as per configuration.",
                    status
                );
            }
            OnCommandError::Propagate => {
                if let Some(code) = status.code() {
                    log::warn!(
                        "Command exited with non-zero status: {}, propagating exit code.",
                        code
                    );
                    std::process::exit(code);
                } else {
                    bail!("Command terminated by signal");
                }
            }
        }
    }

    // Calculate hash again
    let after_hash = calculate_directory_hash(&target, &args.include, &args.exclude)?;
    log::info!("Hash after command run: {}", after_hash);

    // Compare hashes
    if before_hash != after_hash {
        bail!(
            "Hash has changed after running command: {} != {}",
            before_hash,
            after_hash
        );
    }

    // No changes detected
    log::info!("Target hash matches, no changes detected.");
    Ok(())
}

// NOTE: There is more performant library [merkle_hash](https://github.com/hristogochev/merkle_hash) exists,
//       but using our version here for more control over hashing process (hasher, include/exclude patterns, etc.)
// TODO(lasuillard): `DefaultHasher` may change between Rust versions, consider replacing it with more stable hasher
//                   IF speed becomes an issue, for large file handling (BLAKE3 or xxHash)
fn calculate_directory_hash(
    path: &PathBuf,
    include: &[String],
    exclude: &[String],
) -> Result<String> {
    log::debug!(
        "Calculating hash for directory: {}; include: {:?}, exclude: {:?}",
        path.display(),
        include,
        exclude
    );
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; BUFFER_SIZE];
    for path in list_files(&path, &include, &exclude) {
        // ? Should take account directory structure in the hash?
        if path.is_dir() {
            log::debug!("Skipping directory: {}", path.display());
            continue;
        }

        log::debug!("Calculating hash for file: {}", path.display());
        let mut file = File::open(path)?;
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffer[..bytes_read].hash(&mut hasher);
        }
    }
    let hash = hasher.finish();
    let hash_as_hex = format!("{:x}", hash);
    Ok(hash_as_hex)
}
