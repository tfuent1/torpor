use crate::models::request::Request;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> anyhow::Result<Request> {
    let contents = fs::read_to_string(path)?;
    let request: Request = serde_yaml::from_str(&contents)?;
    Ok(request)
}

pub fn save(path: &Path, request: &Request) -> anyhow::Result<()> {
    let contents = serde_yaml::to_string(request)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
