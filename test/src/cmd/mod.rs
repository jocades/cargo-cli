mod one;
use one::One;
mod two;
use two::Two;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    One(One),
    Two(Two),
}

impl Command {
    pub fn execute(&self) -> crate::Result<()> {
        use Command::*;
        match self {
            One(cmd) => cmd.execute(),
            Two(cmd) => cmd.execute(),
        }
    }
}