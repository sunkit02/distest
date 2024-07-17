use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs {
    pub api_key_path: PathBuf,
}

impl Configs {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
