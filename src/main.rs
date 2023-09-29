use clap::{arg, command, Parser, Subcommand};

use cmd::lint;

mod cmd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true, subcommand_required = true)]
#[command(
    help_template = "{usage-heading} {usage} \n {about-section} \n\n{all-args} {tab}\n\nVersion: {version} \nAuthor: {author-with-newline}"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Set log level to debug
    #[arg(long)]
    debug: bool,

    /// Set log level to trace, implies debug
    #[arg(long)]
    trace: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage series of commands, execute on a git hook or manually
    Lint(lint::Args),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Lint(args) => lint::run(args),
    }
}
