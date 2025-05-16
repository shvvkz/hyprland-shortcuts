use crate::VERSION;

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
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

        println!("Updated to version {}", tag);
    } else {
        println!("Already up to date (v{})", VERSION);
    }
    Ok(())
}
