use clap::Parser;

mod cli;
mod format;

use cli::{run_app, Cli};

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_app(cli) {
        eprintln!("{e}");
    }
}
