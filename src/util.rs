//! Utility functions

use std::io::{self, Read, Write};

/// Pause stdin and wait for input.
pub fn pause() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...")?;
    stdout.flush()?;

    let _ = stdin.read(&mut [0u8])?;
    Ok(())
}
