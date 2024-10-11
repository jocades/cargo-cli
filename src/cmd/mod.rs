mod add;
use add::Add;
mod new;
use new::New;

use clap::Subcommand;

#[macro_export]
macro_rules! run {
    ($cmd:expr, $args:expr) => {{
        $cmd.args($args)
            .status()
            .and_then(|status| Ok(status.success()))
    }};

    ($cmd:expr, $($arg:expr),*) => {{
        $crate::run!($cmd, &[$($arg),*])
    }};
}

#[derive(Subcommand)]
pub enum Command {
    New(New),
    Add(Add),
}

impl Command {
    pub fn execute(&self) -> fu::Result<()> {
        use Command::*;
        match self {
            New(cmd) => cmd.execute(),
            Add(cmd) => cmd.execute(),
        }
    }
}
