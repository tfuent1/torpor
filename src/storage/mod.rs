pub mod collection;
pub mod environment;
pub mod request;
pub mod workspace;

use std::fs;
use std::path::Path;

/// Write `contents` to `path` atomically by writing to a sibling temp file
/// and then renaming. The parent directory must already exist.
pub(crate) fn atomic_write(path: &Path, contents: &str) -> anyhow::Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
