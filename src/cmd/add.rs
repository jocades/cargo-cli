use std::fs;

use anyhow::{anyhow, Result};
use clap::Args;
use regex::Regex;

use super::{snake_to_pascal_case, write_command};

#[derive(Args)]
pub struct Add {
    command: String,
}

impl Add {
    pub fn execute(&self) -> Result<()> {
        let mut content = fs::read_to_string("src/cmd/mod.rs")?;
        let name = snake_to_pascal_case(&self.command);

        let import_regex = Regex::new(r"(?m)^use.*?;\n$")?;
        let last_import = import_regex
            .find_iter(&content)
            .last()
            .ok_or(anyhow!("No imports found"))?;
        content.insert_str(
            last_import.end(),
            &format!("\nmod {};\nuse {}::{};\n", self.command, self.command, name),
        );

        let enum_regex = Regex::new(r"pub enum Command \{(?s).*?\n\}")?;
        let enum_match = enum_regex
            .find(&content)
            .ok_or(anyhow!("Command enum not found"))?;
        let new_variant = format!("    {}({}),\n", name, name);
        content.insert_str(enum_match.end() - 1, &new_variant);

        let match_regex = Regex::new(r"match self \{(?s).*?\n        \}")?;
        let match_match = match_regex
            .find(&content)
            .ok_or(anyhow!("match expression not found"))?;

        let new_arm = format!("            {}(cmd) => cmd.execute(),\n", name);
        content.insert_str(match_match.end() - 9, &new_arm);

        fs::write("src/cmd/mod.rs", &content)?;

        write_command(&self.command, &name)?;

        Ok(())
    }
}
