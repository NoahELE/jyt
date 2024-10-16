use std::{fs, path::Path};

use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::format::{json, toml, yaml, JsonValue, TomlValue, YamlValue};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            SubCommand::Json { file, output } => {
                let value: JsonValue = deserialize_by_file_type(&file)?;
                let s = json::serialize(value)?;
                // if output file is given, write to file, otherwise print to stdout
                if let Some(output) = output {
                    fs::write(output, s)?;
                } else {
                    print!("{s}");
                }
            }

            SubCommand::Yaml { file, output } => {
                let value: YamlValue = deserialize_by_file_type(&file)?;
                let s = yaml::serialize(value)?;
                // if output file is given, write to file, otherwise print to stdout
                if let Some(output) = output {
                    fs::write(output, s)?;
                } else {
                    print!("{s}");
                }
            }

            SubCommand::Toml { file, output } => {
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
}

#[derive(Subcommand)]
enum SubCommand {
    /// Convert input to json
    #[command(name = "to-json")]
    Json {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[command(name = "to-yaml")]
    Yaml {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert input to json
    #[command(name = "to-toml")]
    Toml {
        /// The file to parse
        #[arg(short, long)]
        file: String,
        /// The output file
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn deserialize_by_file_type<V>(file: &str) -> Result<V>
where
    V: for<'de> Deserialize<'de>,
{
    let file_path = Path::new(file);
    let content = fs::read_to_string(file_path)?;
    match file_path.extension() {
        Some(ext) => {
            let ext = ext
                .to_str()
                .ok_or(anyhow!("Converting `OsStr` to `&str` failed"))?
                .trim()
                .to_lowercase();
            match ext.as_str() {
                "json" => json::deserialize(&content),
                "yaml" | "yml" => yaml::deserialize(&content),
                "toml" => toml::deserialize(&content),
                _ => bail!("Unknown extension {ext}"),
            }
            .map_err(|e| e.into())
        }
        None => bail!("File {file} does not have a extension"),
    }
}
