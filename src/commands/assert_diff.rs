use std::{fs::File,
          hash::{DefaultHasher, Hash, Hasher},
          io::Read,
          path::{PathBuf, absolute}};

use anyhow::{Result, bail};
use clap::Args;

use crate::{GlobalOpts, utils::fs::list_files};

const BUFFER_SIZE: usize = 8192;

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

    /// Path to the temporary hash file to store and compare the computed hash.
    ///
    /// If file already exists, compute the hash and compare with the existing hash.
    /// Otherwise, create a new hash file with the computed hash.
    ///
    /// If not provided, automatically generates a hash file at OS temporary location.
    #[arg(long, default_value = None)]
    hash_file: Option<PathBuf>,

    /// By default, the hash file is deleted after comparison. If this flag is set,
    /// the hash file will be preserved after comparison.
    #[arg(long, default_value_t = false)]
    preserve_hash_file: bool,
}

pub(crate) fn command(args: CommandArgs, _global_opts: GlobalOpts) -> Result<()> {
    // Prepare arguments
    let target = absolute(PathBuf::from(&args.target))?;
    if !target.exists() {
        bail!("Target path does not exist: {}", target.display());
    }
    let temp_dir = std::env::temp_dir();
    let hash_file = args.hash_file.unwrap_or_else(|| {
        let mut path = temp_dir;
        path.push("assert-diff.hash");
        log::info!("Using hash file at: {}", path.display());
        path
    });
    let preserve_hash_file = args.preserve_hash_file;

    // Calculate directory hash
    log::info!("Calculating directory hash for: {}", target.display());
    let hash = calculate_directory_hash(&target, &args.include, &args.exclude)?;
    log::info!("Directory hash: {}", hash);

    // If hash file does not exist, create it and exit
    if !hash_file.exists() {
        log::info!("Creating new hash file at: {}", hash_file.display());
        std::fs::write(&hash_file, hash)?;
        return Ok(());
    }

    // If hash file exists, read the existing hash and compare
    let existing_hash = std::fs::read_to_string(&hash_file)?;
    log::info!("Existing hash: {}", existing_hash);

    // Compare hashes
    if hash != existing_hash {
        bail!(
            "Directory hash does not match the existing hash: {} != {}",
            hash,
            existing_hash
        );
    }

    // Optionally delete the hash file after comparison
    if !preserve_hash_file {
        log::info!("Deleting hash file at: {}", hash_file.display());
        std::fs::remove_file(&hash_file)?;
    }

    log::info!("Directory hash matches the existing hash.");
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
