mod bind;
mod flags;

use clap::Parser;
use bind::Bind;
use flags::Flags;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args = Flags::parse();

    if args.version {
        println!("hyprland-shortcuts v{}", VERSION);
        return Ok(());
    }

    if args.update {
        if let Err(e) = self_update() {
            eprintln!("❌ Update failed: {}", e);
        }
        return Ok(());
    }

    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    let path = format!("{}/.config/hypr/hyprland.conf", home_dir);
    let raw_lines = read_lines(path)?;

    let binds: Vec<Bind> = raw_lines
        .into_iter()
        .filter(|line| line.contains("bind =") || line.contains("bindm ="))
        .filter_map(|line| Bind::from_line(&line))
        .collect();

    if args.display {
        for comments in binds.iter() {
            if let Some(comment) = &comments.comment {
                println!("{}", comment);
            }
        }
        return Ok(());
    }

    // Default behavior here if needed
    Ok(())
}

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let repo = "shvvkz/hyprland-shortcuts";
    let install_dir = "/usr/local/bin";
    let binary_path = format!("{}/hyprland-shortcuts", install_dir);

    let latest_version_output = Command::new("curl")
        .args([
            "-s",
            &format!("https://api.github.com/repos/{}/releases/latest", repo),
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&latest_version_output.stdout);
    let tag = stdout
        .lines()
        .find(|line| line.contains("\"tag_name\""))
        .and_then(|line| line.split('"').nth(3))
        .ok_or("Failed to get latest version")?;

    if tag != format!("v{}", VERSION) {
        println!("⬇️ New version {} found, updating...", tag);

        let tmp_path = "/tmp/hyprland-shortcuts";
        Command::new("curl")
            .args([
                "-L",
                "-o",
                tmp_path,
                &format!(
                    "https://github.com/{}/releases/download/{}/hyprland-shortcuts",
                    repo, tag
                ),
            ])
            .status()?;

        Command::new("sudo")
            .args(["mv", tmp_path, &binary_path])
            .status()?;
        Command::new("sudo")
            .args(["chmod", "+x", &binary_path])
            .status()?;

        println!("✅ Updated to version {}", tag);
    } else {
        println!("✅ Already up to date (v{})", VERSION);
    }
    Ok(())
}
