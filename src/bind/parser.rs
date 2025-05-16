use crate::utils::file::file_exists;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Bind {
    pub main_key: Option<String>,
    pub key: String,
    pub action: String,
    pub arguments: Option<String>,
    pub comment: Option<String>,
}

impl Bind {
    pub fn from_line(line: &str) -> Option<Self> {
        let line = line.replace("bind =", "").replace("bindm =", "");
        let cleaned_line = line.trim();

        let (bind_part, comment) = match cleaned_line.split_once('#') {
            Some((cmd, cmt)) => (cmd.trim(), Some(cmt.trim().to_string())),
            None => (cleaned_line, None),
        };

        let parts: Vec<&str> = bind_part.split(',').map(|s| s.trim()).collect();
        if parts.len() < 3 {
            return None;
        }

        let main_key = if parts[0].is_empty() {
            None
        } else {
            Some(parts[0].to_string())
        };

        let key = parts[1].to_string();
        let action = parts[2].to_string();

        let arguments = if parts.len() > 3 {
            Some(parts[3..].join(", ").trim().to_string())
        } else {
            None
        };

        Some(Self {
            main_key,
            key,
            action,
            arguments,
            comment,
        })
    }
}

pub fn parse_binds(custom_path: Option<String>) -> io::Result<Vec<Bind>> {
    let path = match custom_path {
        Some(ref p) if file_exists(p) => p.clone(),
        Some(ref p) => {
            eprintln!("Provided path '{}' does not exist. Using default path.", p);
            std::process::exit(1);
        }
        None => default_path(),
    };

    read_lines(path)
}

fn default_path() -> String {
    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    format!("{}/.config/hypr/hyprland.conf", home_dir)
}

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<Bind>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    Ok(lines
        .filter_map(|line| match line {
            Ok(l) if l.contains("bind =") || l.contains("bindm =") => Bind::from_line(&l),
            _ => None,
        })
        .collect())
}
