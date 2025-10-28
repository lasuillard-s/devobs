use std::{fs::File,
          hash::{DefaultHasher, Hash, Hasher},
          io::Read,
          path::PathBuf};

use anyhow::Result;

use crate::utils::fs::list_files;

const BUFFER_SIZE: usize = 8192;

// NOTE: There is more performant library [merkle_hash](https://github.com/hristogochev/merkle_hash) exists,
//       but using our version here for more control over hashing process (hasher, include/exclude patterns, etc.)
// TODO(lasuillard): `DefaultHasher` may change between Rust versions, consider replacing it with more stable hasher
//                   IF speed becomes an issue, for large file handling (BLAKE3 or xxHash)
pub(crate) fn calculate_directory_hash(
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
