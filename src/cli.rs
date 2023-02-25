use std::{fs, path::Path};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use crate::format::{json, toml, yaml, JsonValue, TomlValue, YamlValue};

#[derive(Parser)]
#[clap(version, about)]
pub(crate) struct Cli {
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

pub(crate) fn run_app(cli: Cli) -> Result<()> {
    use SubCommand::*;

    match cli.command {
        Json { file, output } => {
            let value: JsonValue = deserialize_by_file_type(&file)?;
            let s = json::serialize(value)?;
            print!("{s}");
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }

        Yaml { file, output } => {
            let value: YamlValue = deserialize_by_file_type(&file)?;
            let s = yaml::serialize(value)?;
            print!("{s}");
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }

        Toml { file, output } => {
            let value: TomlValue = deserialize_by_file_type(&file)?;
            let s = toml::serialize(value)?;
            print!("{s}");
            if let Some(output) = output {
                fs::write(output, s)?;
            }
        }
    };

    Ok(())
}

fn deserialize_by_file_type<V>(file: &str) -> Result<V>
where
    V: for<'de> serde::Deserialize<'de>,
{
    let file_path = Path::new(file);
    let content = fs::read_to_string(file_path)?;
    match file_path.extension() {
        Some(ext) => {
            let ext = ext.to_str().expect("Converting `OsStr` to `&str` failed");
            match ext {
                "json" => json::deserialize(&content).map_err(Into::into),
                "yaml" | "yml" => yaml::deserialize(&content).map_err(Into::into),
                "toml" => toml::deserialize(&content).map_err(Into::into),
                _ => bail!("Unknown extension {}", ext),
            }
        }
        None => bail!("File {} does not have a extension", file),
    }
}
