mod add;
use add::Add;
mod new;
use new::New;

use clap::Subcommand;

#[macro_export]
macro_rules! cmd {
    ($prog:expr) => {{
        $crate::cmd::finish(&mut std::process::Command::new($prog))
    }};

    ($prog:expr, $args:expr) => {{
        $crate::cmd::finish(std::process::Command::new($prog).args($args))
    }};

    ($prog:expr, $($arg:expr),*) => {{
        $crate::cmd!($prog, &[$($arg),*])
    }};
}

pub(crate) fn finish(cmd: &mut std::process::Command) -> anyhow::Result<()> {
    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => anyhow::bail!("command failed with non-zero exit code"),
        Err(e) => anyhow::bail!("failed to run command: {e}"),
    }
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
