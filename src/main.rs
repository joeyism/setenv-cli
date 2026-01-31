mod config;
mod shell;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::{Config, is_valid_env_var_name};
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
    List,
    Current,
    Edit,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::List) => cmd_list(),
        Some(Commands::Current) => cmd_current(),
        Some(Commands::Edit) => cmd_edit(),
        None => {
            if let Some(profile_name) = cli.profile {
                cmd_switch(&profile_name)
            } else {
                Cli::parse_from(&["setenv", "--help"]);
                Ok(())
            }
        }
    }
}

fn cmd_switch(profile_name: &str) -> Result<()> {
    let config = Config::load()?;
    let profile = config.get_profile(profile_name)
        .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found. Available profiles: {}", 
            profile_name, 
            config.profile_names().join(", ")))?;
    
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
    
    let new_var_names: Vec<String> = profile.env_vars.keys()
        .map(|s| s.to_string())
        .collect();
    let vars_list = new_var_names.join(" ");
    println!("{}", shell.export_var("SETENV_VARS", &vars_list));
    
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
    
    let status = Command::new(&editor)
        .arg(&config_path)
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Editor exited with error");
    }
    
    Ok(())
}

fn which(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
