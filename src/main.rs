use clap::Parser;

mod cli;
mod format;

use cli::{run_cli, Cli};

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_cli(cli) {
        eprintln!("{e}");
    }
}
