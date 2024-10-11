mod add;
use add::Add;
mod new;
use new::New;

use clap::Subcommand;

#[macro_export]
macro_rules! cmd {
    ($prog:expr) => {{
        $crate::finish!(std::process::Command::new($prog))
    }};

    ($prog:expr, $args:expr) => {{
        $crate::finish!(std::process::Command::new($prog).args($args))
    }};

    ($prog:expr, $($arg:expr),*) => {{
        $crate::cmd!($prog, &[$($arg),*])
    }};
}

#[macro_export]
macro_rules! finish {
    ($cmd:expr) => {
        match $cmd.status() {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(anyhow::anyhow!("command failed with non-zero exit code")),
            Err(e) => Err(anyhow::anyhow!("failed to run command: {}", e)),
        }
    };
}

#[derive(Subcommand)]
pub enum Command {
    New(New),
    Add(Add),
}

impl Command {
    pub fn execute(&self) -> anyhow::Result<()> {
        use Command::*;
        match self {
            New(cmd) => cmd.execute(),
            Add(cmd) => cmd.execute(),
        }
    }
}
