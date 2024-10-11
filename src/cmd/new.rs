use std::{
    env, fs,
    process::{self},
};

use clap::Args;

use crate::run;

#[derive(Args)]
pub struct New {
    // Add command-specific arguments here
    name: String,
}

fn snake_to_pascal_case(snake_str: &str) -> String {
    snake_str
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

fn write_main() -> fu::Result<()> {
    let main_rs = "mod cmd;
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
}";

    fs::write("test_main.rs", main_rs)?;
    Ok(())
}

fn write_command(cmd: &str, name: &str) -> fu::Result<()> {
    let content = format!(
        r#"use clap::Args;

#[derive(Args)]
pub struct {name} {{
    // Add command-specific arguments here
}}

impl {name} {{
    pub fn execute(&self) -> crate::Result<()> {{
        // Implement command logic here
        println!("{name} command executed");
        Ok(())
    }}
}}"#
    );
    fs::write(format!("src/cmd/{cmd}.rs"), content)?;
    Ok(())
}

fn write_commands(cmds: &[&str]) -> fu::Result<()> {
    let mut imports = String::new();
    let mut variants = String::new();
    let mut arms = String::new();

    for cmd in cmds {
        let name = snake_to_pascal_case(cmd);
        imports.push_str(&format!("mod {cmd};\nuse {cmd}::{name};\n"));
        variants.push_str(&format!("{name}({name}),\n    "));
        arms.push_str(&format!("{name}(cmd) => cmd.execute(),\n            "));
        write_command(cmd, &name)?;
    }

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
}}"#
    );

    fs::create_dir_all("src/cmd")?;
    fs::write("src/cmd/mod.rs", cmd_mod_rs)?;
    Ok(())
}

impl New {
    pub fn execute(&self) -> fu::Result<()> {
        fs::create_dir_all("test")?;
        env::set_current_dir("test")?;
        /* let mut cmd = process::Command::new("cargo");

                if !run!(cmd, "new", &self.name)? {
                    process::exit(1);
                }

                cmd.current_dir(&self.name);
                if !run!(cmd, "add", "clap", "--features", "derive")? {
                    process::exit(1);
                }
        */

        let cmds = ["one", "two"];

        write_main()?;
        write_commands(&cmds)?;

        Ok(())
    }
}
