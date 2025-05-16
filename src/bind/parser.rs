use crate::bind::bind::Bind;
use crate::utils::file::file_exists;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        .filter_map(Result::ok)
        .filter(|line| line.contains("bind =") || line.contains("bindm ="))
        .filter_map(|line| Bind::from_line(&line))
        .collect())
}
