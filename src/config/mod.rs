pub mod keybinds;
pub mod theme;

pub use keybinds::KeyBinds;
pub use theme::{Theme, by_name};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Name of the active theme. Must match a built-in theme name or a custom
    /// theme file at `~/.config/torpor/themes/<name>.toml`.
    #[serde(default = "default_theme_name")]
    pub theme: String,

    /// Keybind overrides. Any action not listed uses its compiled-in default.
    #[serde(default)]
    pub keybinds: KeyBinds,
}

fn default_theme_name() -> String {
    "default-dark".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme_name(),
            keybinds: KeyBinds::default(),
        }
    }
}

impl Config {
    /// Returns the XDG config directory for Torpor: `~/.config/torpor/`.
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("torpor"))
    }

    /// Returns the path to the main config file.
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|d: PathBuf| d.join("config.toml"))
    }

    /// Loads config from disk. Returns `Config::default()` if the file does
    /// not exist. Errors on parse failure so the user knows something is wrong.
    pub fn load() -> anyhow::Result<Self> {
        let Some(path) = Self::config_path() else {
            return Ok(Self::default());
        };

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file: {}", path.display()))?;

        let config: Self = toml::from_str(&contents)
            .with_context(|| format!("failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Writes the current config to disk, creating directories as needed.
    pub fn save(&self) -> anyhow::Result<()> {
        let Some(path) = Self::config_path() else {
            return Ok(());
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent as &std::path::Path).with_context(|| {
                format!("failed to create config directory: {}", parent.display())
            })?;
        }

        let contents =
            toml::to_string_pretty(self).context("failed to serialize config to TOML")?;

        fs::write(&path, contents)
            .with_context(|| format!("failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Writes the default config file if none exists yet. Safe to call on
    /// every startup — does nothing if the file is already present.
    pub fn ensure_default() -> anyhow::Result<()> {
        let Some(path) = Self::config_path() else {
            return Ok(());
        };
        if path.exists() {
            return Ok(());
        }
        Self::default().save()
    }

    /// Resolves the active `Theme`. Checks
    /// `~/.config/torpor/themes/<name>.toml` first, then falls back to
    /// built-ins.
    pub fn resolve_theme(&self) -> Theme {
        if let Some(dir) = Self::config_dir() {
            let custom_path = dir.join("themes").join(format!("{}.toml", self.theme));
            if custom_path.exists()
                && let Ok(contents) = fs::read_to_string(&custom_path)
                && let Ok(theme) = toml::from_str::<Theme>(&contents)
            {
                return theme;
            }
        }

        by_name(&self.theme)
    }
}
