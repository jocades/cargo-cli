use clap::Subcommand;
use std::fs;

mod add;
use add::Add;

mod new;
use new::New;

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

pub(crate) fn snake_to_pascal_case(snake: &str) -> String {
    snake
        .split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<String>()
}

pub(crate) fn write_command(cmd: &str, name: &str) -> anyhow::Result<()> {
    let content = format!(
        r#"use clap::Args;

#[derive(Args)]
pub struct {name} {{
    // Add command-specific arguments here
}}

impl {name} {{
    pub fn execute(&self) -> crate::Result<()> {{
        // Implement command logic here
        println!("{cmd} command executed");
        Ok(())
    }}
}}
"#
    );
    fs::write(format!("src/cmd/{cmd}.rs"), content)?;
    Ok(())
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
