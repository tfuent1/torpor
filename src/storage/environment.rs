use crate::models::environment::Environment;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> anyhow::Result<Environment> {
    let contents = fs::read_to_string(path)?;
    let environment: Environment = serde_yaml::from_str(&contents)?;
    Ok(environment)
}

pub fn save(path: &Path, environment: &Environment) -> anyhow::Result<()> {
    let contents = serde_yaml::to_string(environment)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
