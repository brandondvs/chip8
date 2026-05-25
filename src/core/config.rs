use config::{Config as Conf, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmulatorConfig {
    rom_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    emulator: EmulatorConfig,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        Conf::builder()
            .add_source(File::with_name("config"))
            .build()?
            .try_deserialize::<Config>()
    }

    pub fn rom_file(&self) -> &str {
        &self.emulator.rom_file
    }
}
