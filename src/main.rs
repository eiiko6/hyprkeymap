use clap::Parser;
use colored::Colorize;
use core::{Search, display_keybind, display_layout, get_keybinds};
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();

    match args.action {
        cli::Action::Layout(layout_args) => {
            if args.verbose {
                println!("Displaying layer {}", layout_args.layer.bright_blue());
            }

            let keybinds = match get_keybinds(args.paths, Search::All, args.verbose) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Failed to get keybinds from config files: {e}");
                    return;
                }
            };

            // Temporary alternative to an actual display
            display_layout(keybinds, layout_args);
        }
        cli::Action::Check(check_args) => {
            if args.verbose {
                println!("Checking key {}", check_args.key.bright_blue(),);
            }

            let keybinds = match get_keybinds(
                args.paths,
                Search::Key(check_args.key.clone()),
                args.verbose,
            ) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Failed to get keybinds from config files: {e}");
                    return;
                }
            };

            // Temporary alternative to an actual display
            display_keybind(keybinds, check_args);
        }
    }
}
