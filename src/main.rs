mod config;
mod shell;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::{is_valid_env_var_name, Config};
use shell::Shell;
use std::env;
use std::process::Command;

#[derive(Parser)]
#[command(name = "setenv")]
#[command(about = "Manage environment variable profiles", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(value_name = "PROFILE")]
    profile: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available profiles
    List,
    /// Show the currently active profile
    Current,
    /// Open the config file in your editor
    Edit,
    /// Create a new empty profile
    New {
        #[arg(value_name = "PROFILE")]
        profile: String,
    },
    /// Add a variable to an existing profile
    Add {
        #[arg(value_name = "PROFILE")]
        profile: String,
        #[arg(value_name = "KEY")]
        key: String,
        #[arg(value_name = "VALUE")]
        value: String,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    if std::env::args().len() == 1 {
        return cmd_no_args();
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List) => cmd_list(),
        Some(Commands::Current) => cmd_current(),
        Some(Commands::Edit) => cmd_edit(),
        Some(Commands::New { profile }) => cmd_new(&profile),
        Some(Commands::Add {
            profile,
            key,
            value,
        }) => cmd_add(&profile, &key, &value),
        None => {
            if let Some(profile_name) = cli.profile {
                cmd_switch(&profile_name)
            } else {
                cmd_no_args()
            }
        }
    }
}

fn cmd_switch(profile_name: &str) -> Result<()> {
    let config = Config::load()?;
    let profile = config.get_profile(profile_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile '{}' not found. Available profiles: {}",
            profile_name,
            config.profile_names().join(", ")
        )
    })?;

    let shell = Shell::detect();

    let current_vars = env::var("SETENV_VARS").unwrap_or_default();
    let old_vars: Vec<String> = current_vars
        .split_whitespace()
        .filter(|name| is_valid_env_var_name(name))
        .filter(|name| *name != "SETENV_VARS" && *name != "SETENV_PROFILE")
        .map(|s| s.to_string())
        .collect();

    if let Some(unset_cmd) = shell.unset_vars(&old_vars) {
        println!("{}", unset_cmd);
    }

    for (name, value) in &profile.env_vars {
        println!("{}", shell.export_var(name, value));
    }

    println!("{}", shell.export_var("SETENV_PROFILE", profile_name));

    let new_var_names: Vec<String> = profile.env_vars.keys().map(|s| s.to_string()).collect();
    let vars_list = new_var_names.join(" ");
    println!("{}", shell.export_var("SETENV_VARS", &vars_list));

    Ok(())
}

fn cmd_no_args() -> Result<()> {
    let config = Config::load()?;
    let current_profile = env::var("SETENV_PROFILE").ok();

    println!("setenv - Environment variable profile manager\n");

    println!("Available profiles:");
    for name in config.profile_names() {
        if Some(name.to_string()) == current_profile {
            println!("  * {} (active)", name);
        } else {
            println!("    {}", name);
        }
    }

    println!("\nQuick start:");
    println!("  setenv <profile>     Switch to a profile");
    println!("  setenv list          List all profiles");
    println!("  setenv new <name>    Create a new profile");
    println!("  setenv add <profile> <KEY> <VALUE>");
    println!("                       Add a variable to a profile");
    println!("  setenv edit          Edit config file");
    println!("  setenv --help        Show detailed help");

    Ok(())
}

fn cmd_list() -> Result<()> {
    let config = Config::load()?;
    let current_profile = env::var("SETENV_PROFILE").ok();

    println!("Available profiles:");
    for name in config.profile_names() {
        if Some(name.to_string()) == current_profile {
            println!("  * {}", name);
        } else {
            println!("    {}", name);
        }
    }

    Ok(())
}

fn cmd_current() -> Result<()> {
    match env::var("SETENV_PROFILE") {
        Ok(profile) => {
            println!("{}", profile);
        }
        Err(_) => {
            println!("No profile active");
        }
    }

    Ok(())
}

fn cmd_edit() -> Result<()> {
    let config_path = Config::config_path()?;

    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| {
            if which("vim") {
                "vim".to_string()
            } else if which("vi") {
                "vi".to_string()
            } else if which("nano") {
                "nano".to_string()
            } else {
                "vi".to_string()
            }
        });

    let status = Command::new(&editor).arg(&config_path).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with error");
    }

    Ok(())
}

fn cmd_new(profile_name: &str) -> Result<()> {
    let mut config = Config::load()?;

    if config.get_profile(profile_name).is_some() {
        anyhow::bail!(
            "Profile '{}' already exists. Use 'setenv add' to add variables to it.",
            profile_name
        );
    }

    config.profiles.insert(
        profile_name.to_string(),
        config::Profile {
            env_vars: std::collections::HashMap::new(),
        },
    );

    config.save()?;

    println!("Created new profile '{}'", profile_name);
    println!("Add variables with: setenv add {} KEY VALUE", profile_name);

    Ok(())
}

fn cmd_add(profile_name: &str, key: &str, value: &str) -> Result<()> {
    let mut config = Config::load()?;

    let profile = config.profiles.get_mut(profile_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile '{}' not found. Create it first with: setenv new {}",
            profile_name,
            profile_name
        )
    })?;

    if !is_valid_env_var_name(key) {
        anyhow::bail!("Invalid variable name '{}'. Variable names must start with a letter or underscore and contain only letters, numbers, or underscores.", key);
    }

    if key == "SETENV_VARS" || key == "SETENV_PROFILE" {
        anyhow::bail!("Cannot use reserved variable name '{}'", key);
    }

    profile.env_vars.insert(key.to_string(), value.to_string());

    config.save()?;

    println!("Added {}=\"{}\" to profile '{}'", key, value, profile_name);

    Ok(())
}

fn which(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
