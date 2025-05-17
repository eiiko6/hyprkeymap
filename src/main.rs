use clap::Parser;
use colored::Colorize;
use core::get_keybinds;
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();

    if args.verbose {
        for path in &args.paths {
            println!("Using {}", path.bright_yellow());
        }
    }

    match args.action {
        cli::Action::Layout(layout_args) => {
            if args.verbose {
                println!("Displaying layer {}", layout_args.layer.bright_blue());
            }

            let keybinds = match get_keybinds(args.paths, args.verbose) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Failed to get keybinds from config files: {e}");
                    return;
                }
            };

            // Temporary alternative to an actual display
            for keybind in keybinds.iter() {
                if keybind.layer == layout_args.layer {
                    println!(
                        "-> {} {} {} {} {} {}",
                        keybind.modifier.red(),
                        keybind.layer.blue(),
                        keybind.key.magenta(),
                        keybind.action.yellow(),
                        keybind.command,
                        keybind.comment.bright_green()
                    );
                }
            }
        }
        cli::Action::Check(check_args) => {
            if args.verbose {
                println!("Checking key {}", check_args.key.bright_blue(),);
            }
        }
    }
}
