// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use wsrx_desktop::logging;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    let (_console_guard, _file_guard) = logging::setup()?;

    // Run the application (cross-platform)
    wsrx_desktop::run_app()?;

    Ok(())
}