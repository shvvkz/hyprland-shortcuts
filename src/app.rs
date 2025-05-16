use crate::bind::parser::parse_binds;
use crate::commands::{display::display_comments, update::update, version::print_version};
use crate::flags::cli::Flags;

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
