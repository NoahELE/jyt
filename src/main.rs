use clap::Parser;

mod cli;
mod format;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.run() {
        eprintln!("{e}");
    }
}
