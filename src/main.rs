mod app;
mod bind;
mod commands;
mod flags;
mod utils;

use crate::flags::cli::Flags;
use app::run;
use clap::Parser;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let flags = Flags::parse();

    if let Err(e) = run(flags) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
