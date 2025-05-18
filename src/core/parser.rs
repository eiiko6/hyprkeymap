use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Keymap {
    pub modifier: String,
    pub layer: String,
    pub key: String,
    pub action: String,
    pub command: String,
    pub comment: String,
}

#[derive(Clone)]
pub enum Search {
    All,
    Key(String),
}

/// Get Keymap structs from paths
pub fn get_keybinds(
    paths: Vec<String>,
    search: Search,
    verbose: bool,
) -> std::io::Result<Vec<Keymap>> {
    let mut keybinds = Vec::new();

    for path in paths {
        if verbose {
            println!("---|> Reading file {}", path);
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            if let Some(keybind) = parse_keymap(line, search.clone()) {
                keybinds.push(keybind);
            }
        }
    }

    Ok(keybinds)
}

/// Convert a string into a Keymap struct
fn parse_keymap(line: String, search: Search) -> Option<Keymap> {
    // Filter binding lines
    let line = line.trim().strip_prefix("bind")?.split_once('=')?.1.trim();

    let (line, comment) = match line.rfind('#') {
        Some(pos) => (line[..pos].to_string(), line[pos + 1..].trim().to_string()),
        None => (line.to_string(), String::new()),
    };

    // Split into parts
    let parts: Vec<&str> = line.split(',').map(|x| x.trim()).collect();
    if parts.len() != 4 {
        return None;
    }

    let (modifier, layer) = match parts[0].split_once(' ') {
        Some((modifier, layer)) => (modifier.trim(), layer.to_lowercase()),
        None => (parts[0], "base".to_string()),
    };

    // Skip if not the searched key
    if let Search::Key(k) = search {
        if parts[1].to_uppercase() != k.to_uppercase() {
            return None;
        }
    }

    Some(Keymap {
        modifier: modifier.to_string(),
        layer: layer.to_string(),
        key: parts[1].to_string(),
        action: parts[2].to_string(),
        command: parts[3].to_string(),
        comment: comment.to_string(),
    })
}
