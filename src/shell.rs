use std::env;

/// Supported shell types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    /// Detect shell from SHELL environment variable
    pub fn detect() -> Self {
        env::var("SHELL")
            .ok()
            .and_then(|shell_path| {
                if shell_path.contains("fish") {
                    Some(Shell::Fish)
                } else if shell_path.contains("zsh") {
                    Some(Shell::Zsh)
                } else if shell_path.contains("bash") {
                    Some(Shell::Bash)
                } else {
                    None
                }
            })
            .unwrap_or(Shell::Bash)
    }

    /// Generate export command for a variable
    pub fn export_var(&self, name: &str, value: &str) -> String {
        let escaped_value = shell_escape(value);
        match self {
            Shell::Bash | Shell::Zsh => {
                format!("export {}=\"{}\"", name, escaped_value)
            }
            Shell::Fish => {
                format!("set -gx {} \"{}\"", name, escaped_value)
            }
        }
    }

    /// Generate unset command for multiple variables
    pub fn unset_vars(&self, names: &[String]) -> Option<String> {
        if names.is_empty() {
            return None;
        }

        Some(match self {
            Shell::Bash | Shell::Zsh => {
                format!("unset {}", names.join(" "))
            }
            Shell::Fish => {
                format!("set -eg {}", names.join(" "))
            }
        })
    }
}

/// Escape special characters for shell
fn shell_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_bash() {
        let shell = Shell::Bash;
        assert_eq!(shell.export_var("KEY", "value"), "export KEY=\"value\"");
    }

    #[test]
    fn test_export_fish() {
        let shell = Shell::Fish;
        assert_eq!(shell.export_var("KEY", "value"), "set -gx KEY \"value\"");
    }

    #[test]
    fn test_unset_bash() {
        let shell = Shell::Bash;
        let vars = vec!["VAR1".to_string(), "VAR2".to_string()];
        assert_eq!(shell.unset_vars(&vars), Some("unset VAR1 VAR2".to_string()));
    }

    #[test]
    fn test_unset_fish() {
        let shell = Shell::Fish;
        let vars = vec!["VAR1".to_string(), "VAR2".to_string()];
        assert_eq!(
            shell.unset_vars(&vars),
            Some("set -eg VAR1 VAR2".to_string())
        );
    }

    #[test]
    fn test_shell_escape() {
        assert_eq!(shell_escape("simple"), "simple");
        assert_eq!(shell_escape("with\"quotes"), "with\\\"quotes");
        assert_eq!(shell_escape("with$dollar"), "with\\$dollar");
        assert_eq!(shell_escape("with\\backslash"), "with\\\\backslash");
    }
}
