use std::{env, fs, path::PathBuf};

use anyhow::Context;
use clap::{command, Parser};

use crate::configs::{Configs, APP_NAME, DEFAULT_CONFIG_PATH};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct RawArgs {
    /// Address to start the calculations from
    #[arg(short, long)]
    from: String,

    /// Address of destination
    #[arg(short, long)]
    destination: String,

    /// Path to api key file
    #[arg(short('k'), long)]
    api_key: Option<String>,
}

#[derive(Debug)]
pub struct CliArgs {
    pub from: String,
    pub dest: String,
    pub api_key: String,
}

impl TryFrom<RawArgs> for CliArgs {
    type Error = anyhow::Error;

    fn try_from(raw_args: RawArgs) -> std::result::Result<Self, Self::Error> {
        let RawArgs {
            from,
            destination,
            api_key,
        } = raw_args;

        let api_key_path = if let Some(api_key_path) = api_key {
            PathBuf::from(api_key_path)
        } else {
            let user = env::var("USER")?;

            // Get configs
            let config_path = format!("/home/{user}/{DEFAULT_CONFIG_PATH}/{APP_NAME}/configs.toml");
            let configs = Configs::new(&config_path).context("failed to build configs")?;

            configs.api_key_path.context("no api key specified")?
        };

        let api_key = fs::read_to_string(api_key_path).context("failed to read api key")?;

        Ok(Self {
            from,
            dest: destination,
            api_key,
        })
    }
}

pub fn parse_cli_args() -> anyhow::Result<CliArgs> {
    CliArgs::try_from(RawArgs::parse())
}
