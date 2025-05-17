use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
use std::path::Path;

/// Utility to manage your Hyprland key bindings
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,

    /// Hyprland config file paths, separated by commas
    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "~/.config/hypr/hyprland.conf",
        value_parser = file_exists
    )]
    pub paths: Vec<String>,

    /// Activate verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Display the whole layout
    Layout(LayoutArgs),
    /// Display the bindings associated to a key
    Check(CheckArgs),
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    pub key: String,
}

#[derive(Args, Debug)]
pub struct LayoutArgs {
    pub layer: String,
}

fn file_exists(path: &str) -> Result<String, String> {
    let expanded = if let Some(home) = home_dir() {
        path.replace("~/", &format!("{}/", home.display()))
    } else {
        return Err("could not determine home directory".to_string());
    };

    if Path::new(&expanded).exists() {
        Ok(expanded)
    } else {
        Err("file does not exist".to_string())
    }
}
