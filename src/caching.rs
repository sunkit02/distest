use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    configs::{APP_NAME, DEFAULT_CACHE_PATH},
    data::{DistanceQuery, DistanceResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cache {
    entries: HashMap<DistanceQuery, DistanceResponse>,
}

impl Cache {
    pub fn insert(&mut self, key: DistanceQuery, value: DistanceResponse) {
        let _ = self.entries.insert(key, value);
    }

    pub fn get(&self, key: &DistanceQuery) -> Option<&DistanceResponse> {
        self.entries.get(key)
    }

    pub fn contains_key(&self, key: &DistanceQuery) -> bool {
        self.entries.contains_key(key)
    }
}

pub async fn get_cache(path: &str) -> Cache {
    let cache = match std::fs::read_to_string(path) {
        Ok(cache) => {
            // Ensure that parsing of empty cache file doesn't fail
            if cache.is_empty() {
                ron::to_string(&Cache::default()).expect("failed to serialize cache")
            } else {
                cache
            }
        }
        Err(_) => {
            eprintln!("Creating cache file at '{path}'");

            let pathbuf = PathBuf::from(get_default_cache_path());
            let cache_dir = pathbuf
                .parent()
                .expect(&format!("invalid cache path '{path}'"));

            // Create cache file according to path
            match fs::create_dir_all(cache_dir).await {
                Ok(_) => {
                    if let Err(e) = fs::File::create(path).await {
                        eprintln!("WARN: failed to crate cache due to error -> {e}");
                    }
                }
                Err(e) => eprintln!("WARN: failed to crate cache due to error -> {e}"),
            };

            ron::to_string(&Cache::default()).expect("failed to serialize cache")
        }
    };
    ron::from_str(&cache).expect("cache corrupted")
}

pub fn get_default_cache_path() -> String {
    let user = std::env::var("USER").expect("failed to read USER environment variable");
    format!("/home/{user}/{DEFAULT_CACHE_PATH}/{APP_NAME}/cache.ron")
}

pub async fn flush_cache(path: &str, cache: &Cache) {
    let mut file = fs::File::options()
        .write(true)
        .open(path)
        .await
        .expect(&format!("failed to open cache at '{path}'"));

    let cache = ron::to_string(&cache).expect("failed to serialize cache");

    file.write_all(cache.as_bytes())
        .await
        .expect(&format!("failed to write to cache at '{path}'"));

    file.flush().await.expect("failed to flush cache to disk");
}
