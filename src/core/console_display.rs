use crate::cli::{CheckArgs, LayoutArgs};
use crate::core::parser::Keymap;
use colored::Colorize;
use std::collections::HashSet;

pub fn display_layout(keybinds: Vec<Keymap>, layout_args: LayoutArgs) {
    // A 2D keyboard layout, each sub-Vec is a row
    let layout = vec![
        vec![
            "ESC", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
        ],
        vec![
            "~",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "0",
            "-",
            "=",
            "BACKSPACE",
        ],
        vec![
            "TAB", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
        ],
        vec![
            "CAPS", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "ENTER",
        ],
        vec![
            "SHIFT", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "SHIFT",
        ],
        vec!["", "CTRL", "ALT", "", "SPACE", "", "ALT", "CTRL"],
        vec!["", "", "", "LEFT", "UP", "DOWN", "RIGHT"],
    ];

    let used_keys: HashSet<String> = keybinds
        .iter()
        .filter(|k| k.layer == layout_args.layer)
        .map(|k| k.key.to_uppercase())
        .collect();

    for row in layout {
        for key in row {
            let display = if key.is_empty() {
                " ".to_string()
            } else if used_keys.contains(key) {
                key.blue().bold().to_string()
            } else {
                key.dimmed().to_string()
            };
            print!("{}  ", display);
        }
        println!();
    }
}

pub fn display_keybind(keybinds: Vec<Keymap>, check_args: CheckArgs) {
    for keybind in keybinds.iter() {
        if keybind.layer == check_args.layer {
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
