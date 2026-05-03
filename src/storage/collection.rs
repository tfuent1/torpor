use crate::models::collection::Collection;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> anyhow::Result<Collection> {
    let contents = fs::read_to_string(path)?;
    let collection: Collection = serde_yaml::from_str(&contents)?;
    Ok(collection)
}

pub fn save(path: &Path, collection: &Collection) -> anyhow::Result<()> {
    let contents = serde_yaml::to_string(collection)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
