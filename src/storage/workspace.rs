use crate::models::workspace::Workspace;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> anyhow::Result<Workspace> {
    let contents = fs::read_to_string(path)?;
    let workspace: Workspace = serde_yaml::from_str(&contents)?;
    Ok(workspace)
}

pub fn save(path: &Path, workspace: &Workspace) -> anyhow::Result<()> {
    let contents = serde_yaml::to_string(workspace)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
