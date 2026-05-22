pub mod cli;
pub mod event;
pub mod log_io;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
