use clap::Parser;
use colored::Colorize;
use hyprkeymap::get_keybinds;
mod args;
mod hyprkeymap;

fn main() {
    let args = args::Cli::parse();

    if args.verbose {
        for path in &args.paths {
            println!("Using {}", path.bright_yellow());
        }
    }

    match args.action {
        args::Action::Layout(layout_args) => {
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
        args::Action::Check(check_args) => {
            if args.verbose {
                println!("Checking key {}", check_args.key.bright_blue(),);
            }
        }
    }
}
