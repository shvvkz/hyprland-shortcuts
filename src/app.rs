use crate::flags::cli::Flags;
use crate::bind::parser::parse_binds;
use crate::commands::{display::display_comments, version::print_version, update::update};

pub fn run(flags: Flags) -> Result<(), Box<dyn std::error::Error>> {
    if flags.version {
        print_version();
        return Ok(());
    }

    if flags.update {
        update()?;
        return Ok(());
    }

    let binds = parse_binds(flags.path.clone())?;

    if flags.display {
        display_comments(&binds);
        return Ok(());
    }

    // Default behavior if needed
    Ok(())
}
