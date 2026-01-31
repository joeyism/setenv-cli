use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".setenv";
const CONFIG_FILE: &str = "config.toml";

/// Configuration structure containing all profiles
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

/// A profile containing environment variables
#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(flatten)]
    pub env_vars: HashMap<String, String>,
}

impl Config {
    /// Get the config file path (~/.setenv/config.toml)
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    /// Load config from file, creating default if it doesn't exist
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            let config = Self::default_config();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        config.validate()?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Create a default configuration
    fn default_config() -> Self {
        let mut profiles = HashMap::new();

        let mut default_vars = HashMap::new();
        default_vars.insert("EXAMPLE_VAR".to_string(), "example_value".to_string());
        default_vars.insert("ANOTHER_VAR".to_string(), "another_value".to_string());

        profiles.insert(
            "default".to_string(),
            Profile {
                env_vars: default_vars,
            },
        );

        Config { profiles }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        for profile_name in self.profiles.keys() {
            if !is_valid_profile_name(profile_name) {
                bail!("Invalid profile name: '{}'. Profile names must contain only alphanumeric characters, underscores, or dashes.", profile_name);
            }
        }

        for (profile_name, profile) in &self.profiles {
            for var_name in profile.env_vars.keys() {
                if !is_valid_env_var_name(var_name) {
                    bail!("Invalid environment variable name '{}' in profile '{}'. Variable names must start with a letter or underscore and contain only letters, numbers, or underscores.", var_name, profile_name);
                }

                if var_name == "SETENV_VARS" || var_name == "SETENV_PROFILE" {
                    bail!("Reserved variable name '{}' in profile '{}'. SETENV_VARS and SETENV_PROFILE are reserved for internal use.", var_name, profile_name);
                }
            }
        }

        Ok(())
    }

    /// Get a profile by name
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// List all profile names
    pub fn profile_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.profiles.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
}

/// Validate profile name (alphanumeric + underscore/dash)
fn is_valid_profile_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Validate environment variable name (valid shell identifier)
/// Must match: ^[a-zA-Z_][a-zA-Z0-9_]*$
pub fn is_valid_env_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile_names() {
        assert!(is_valid_profile_name("default"));
        assert!(is_valid_profile_name("backend"));
        assert!(is_valid_profile_name("my_profile"));
        assert!(is_valid_profile_name("my-profile"));
        assert!(is_valid_profile_name("profile123"));
    }

    #[test]
    fn test_invalid_profile_names() {
        assert!(!is_valid_profile_name(""));
        assert!(!is_valid_profile_name("my profile"));
        assert!(!is_valid_profile_name("my.profile"));
        assert!(!is_valid_profile_name("my/profile"));
    }

    #[test]
    fn test_valid_env_var_names() {
        assert!(is_valid_env_var_name("API_KEY"));
        assert!(is_valid_env_var_name("_PRIVATE"));
        assert!(is_valid_env_var_name("VAR123"));
        assert!(is_valid_env_var_name("a"));
        assert!(is_valid_env_var_name("_"));
    }

    #[test]
    fn test_invalid_env_var_names() {
        assert!(!is_valid_env_var_name(""));
        assert!(!is_valid_env_var_name("123VAR"));
        assert!(!is_valid_env_var_name("MY-VAR"));
        assert!(!is_valid_env_var_name("MY VAR"));
        assert!(!is_valid_env_var_name("MY.VAR"));
    }
}
