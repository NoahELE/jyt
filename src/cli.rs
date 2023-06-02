use std::{fs, path::Path};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use crate::format::{json, toml, yaml, JsonValue, TomlValue, YamlValue};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum SubCommand {
    /// Convert input to json
    #[command(name = "to-json")]
    ToJson {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[command(name = "to-yaml")]
    ToYaml {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[command(name = "to-toml")]
    ToToml {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },
}

pub fn run_cli(cli: Cli) -> Result<()> {
    use SubCommand::*;

    match cli.command {
        ToJson { file, output } => {
            let value: JsonValue = deserialize_by_file_type(&file)?;
            let s = json::serialize(value)?;
            // if output file is given, write to file, otherwise print to stdout
            if let Some(output) = output {
                fs::write(output, s)?;
            } else {
                print!("{s}");
            }
        }

        ToYaml { file, output } => {
            let value: YamlValue = deserialize_by_file_type(&file)?;
            let s = yaml::serialize(value)?;
            // if output file is given, write to file, otherwise print to stdout
            if let Some(output) = output {
                fs::write(output, s)?;
            } else {
                print!("{s}");
            }
        }

        ToToml { file, output } => {
            let value: TomlValue = deserialize_by_file_type(&file)?;
            let s = toml::serialize(value)?;
            // if output file is given, write to file, otherwise print to stdout
            if let Some(output) = output {
                fs::write(output, s)?;
            } else {
                print!("{s}");
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
