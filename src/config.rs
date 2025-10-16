use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub use_vulkan: bool,
    pub fps_limit: u32,
    // Add more config options like resolution, scaling, etc.
}

pub fn load_config(path: &Path) -> anyhow::Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Config = toml::from_slice(&fs::read(path)?)?;
    Ok(config)
}

pub fn save_config(config: &Config, path: &Path) -> anyhow::Result<()> {
    let toml_str = toml::to_string(config)?;
    let mut file = File::create(path)?;
    file.write_all(toml_str.as_bytes())?;
    Ok(())
}
