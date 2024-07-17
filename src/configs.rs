use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const DEFAULT_CONFIG_PATH: &str = ".config";
pub const DEFAULT_CACHE_PATH: &str = ".cache";

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs {
    pub caching: Option<Caching>,
    pub api_key_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Caching {
    path: PathBuf,
}

impl Configs {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
