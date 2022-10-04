use std::{fs, path::Path};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

mod formats;
use formats::{json, toml, yaml, JsonValue, TomlValue, YamlValue};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Convert input to json
    #[clap(name = "to-json")]
    Json {
        /// The file to parse
        #[clap(short, long)]
        file: String,
        /// The output file
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[clap(name = "to-yaml")]
    Yaml {
        /// The file to parse
        #[clap(short, long)]
        file: String,
        /// The output file
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[clap(name = "to-toml")]
    Toml {
        /// The file to parse
        #[clap(short, long)]
        file: String,
        /// The output file
        #[clap(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_app(cli) {
        println!("{}", e);
    }
}

fn run_app(cli: Cli) -> Result<()> {
    use SubCommand::*;

    match cli.command {
        Json { file, output } => {
            let value: JsonValue = deserialize_by_type(&file)?;
            let s = json::serialize(value)?;
            print!("{}", s);
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }
        Yaml { file, output } => {
            let value: YamlValue = deserialize_by_type(&file)?;
            let s = yaml::serialize(value)?;
            print!("{}", s);
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }
        Toml { file, output } => {
            let value: TomlValue = deserialize_by_type(&file)?;
            let s = toml::serialize(value)?;
            print!("{}", s);
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }
    };
    Ok(())
}

fn deserialize_by_type<V>(file: &str) -> Result<V>
where
    V: for<'de> serde::Deserialize<'de>,
{
    let path = Path::new(file);
    let content = fs::read_to_string(path)?;
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_str().expect("Converting `OsStr` to `&str` failed");
            match ext {
                "json" => json::deserialize(&content).map_err(|e| e.into()),
                "yaml" | "yml" => yaml::deserialize(&content).map_err(|e| e.into()),
                "toml" => toml::deserialize(&content).map_err(|e| e.into()),
                _ => bail!("Unknown extension ({})", ext),
            }
        }
        None => bail!("File ({}) do not have a extension", file),
    }
}
