mod command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use command::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    if handle_args() {
        return Ok(());
    }

    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    let path = format!("{}/.config/hypr/hyprland.conf", home_dir);
    let raw_lines = read_lines(path)?;

    let comments: Vec<String> = raw_lines
        .into_iter()
        .filter(|line| line.contains("bind =") || line.contains("bindm ="))
        .filter_map(|line| Command::from_line(&line))
        .filter_map(|cmd| cmd.comment)
        .collect();

    for comment in comments {
        println!("{}", comment);
    }

    Ok(())
}

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines().filter_map(Result::ok).collect())
}

fn handle_args() -> bool {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("hyprland-shortcuts v{}", VERSION);
        return true;
    }

    if args.contains(&"-u".to_string()) || args.contains(&"--update".to_string()) {
        if let Err(e) = self_update() {
            eprintln!("❌ Update failed: {}", e);
        }
        return true;
    }

    false
}

fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let repo = "shvvkz/hyprland-shortcuts";
    let install_dir = "/usr/local/bin"; // ✅ Nouveau chemin global
    let binary_path = format!("{}/hyprland-shortcuts", install_dir);

    let latest_version_output = Command::new("curl")
        .args(&[
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

        // Télécharger dans /tmp puis déplacer avec sudo
        let tmp_path = "/tmp/hyprland-shortcuts";
        Command::new("curl")
            .args(&[
                "-L",
                "-o",
                tmp_path,
                &format!(
                    "https://github.com/{}/releases/download/{}/hyprland-shortcuts-x86_64",
                    repo, tag
                ),
            ])
            .status()?;

        // Déplacer et rendre exécutable
        Command::new("sudo")
            .args(&["mv", tmp_path, &binary_path])
            .status()?;
        Command::new("sudo")
            .args(&["chmod", "+x", &binary_path])
            .status()?;

        println!("✅ Updated to version {}", tag);
    } else {
        println!("✅ Already up to date (v{})", VERSION);
    }

    Ok(())
}
