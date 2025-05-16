mod app;
mod flags;
mod bind;
mod commands;
mod utils;

use app::run;
use crate::flags::cli::Flags;
use clap::Parser;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let flags = Flags::parse();

    if let Err(e) = run(flags) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
