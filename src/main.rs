use clap::Parser;
use colored::Colorize;
mod args;

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
        }
        args::Action::Check(check_args) => {
            if args.verbose {
                println!("Checking key {}", check_args.key.bright_blue(),);
            }
        }
    }
}
