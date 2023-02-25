use clap::Parser;

mod cli;
use cli::{run_app, Cli};
mod format;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_app(cli) {
        eprintln!("{e}");
    }
}
