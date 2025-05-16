use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hyprland-shortcuts", version, disable_version_flag = true, about = "Manage Hyprland Shortcuts easily")]
pub struct Flags {
    /// Show version
    #[arg(short = 'v', long = "version")]
    pub version: bool,

    /// Trigger self-update
    #[arg(short = 'u', long = "update")]
    pub update: bool,

    /// Display all comments found
    #[arg(short = 'd', long = "display")]
    pub display: bool,
}
