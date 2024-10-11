mod cmd;
use anyhow::Result;
use clap::Parser;

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
