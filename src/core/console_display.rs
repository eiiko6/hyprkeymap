use crate::cli::LayoutArgs;
use crate::core::parser::Keymap;
use colored::Colorize;

pub fn display_layout(keybinds: Vec<Keymap>, layout_args: LayoutArgs) {
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

pub fn display_keybind(keybinds: Vec<Keymap>) {
    for keybind in keybinds.iter() {
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
