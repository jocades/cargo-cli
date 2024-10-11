mod cmd;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cmd::Command,
}

fn main() -> fu::Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
