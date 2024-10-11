use std::{env, fs};

use anyhow::Result;
use clap::{ArgAction, Args};

use crate::cmd;

#[derive(Args)]
pub struct New {
    name: String,

    #[arg(action = ArgAction::Append)]
    commands: Vec<String>,
}

impl New {
    pub fn execute(&self) -> Result<()> {
        cmd!("cargo", "new", &self.name)?;
        env::set_current_dir(&self.name)?;
        cmd!("cargo", "add", "clap", "--features", "derive")?;

        write_main()?;
        write_commands(&self.commands)?;

        Ok(())
    }
}

fn snake_to_pascal_case(snake: &str) -> String {
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

const MAIN_RS: &'static str = r#"mod cmd;
use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
"#;

fn write_main() -> Result<()> {
    fs::write("src/main.rs", MAIN_RS)?;
    Ok(())
}

fn write_command(cmd: &str, name: &str) -> Result<()> {
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

fn write_commands(cmds: &[String]) -> Result<()> {
    fs::create_dir_all("src/cmd")?;
    let mut imports = String::new();
    let mut variants = String::new();
    let mut arms = String::new();

    for cmd in cmds {
        let name = snake_to_pascal_case(cmd);
        imports.push_str(&format!("mod {cmd};\nuse {cmd}::{name};\n\n"));
        variants.push_str(&format!("{name}({name}),\n    "));
        arms.push_str(&format!("{name}(cmd) => cmd.execute(),\n            "));
        write_command(cmd, &name)?;
    }

    let imports = imports.trim_end();
    let variants = variants.trim_end();
    let arms = arms.trim_end();

    let cmd_mod_rs = format!(
        r#"{imports}

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {{
    {variants}
}}

impl Command {{
    pub fn execute(&self) -> crate::Result<()> {{
        use Command::*;
        match self {{
            {arms}
        }}
    }}
}}
"#
    );

    fs::write("src/cmd/mod.rs", cmd_mod_rs)?;
    Ok(())
}
