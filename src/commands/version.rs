pub fn print_version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("hyprland-shortcuts v{}", VERSION);
}
