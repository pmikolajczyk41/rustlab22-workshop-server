use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct ApplicationSettings {
    pub port: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct SwapiSettings {
    pub base_url: String,
    pub timeout_milliseconds: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub swapi: SwapiSettings,
}

impl Settings {
    pub fn read(path: &Path) -> anyhow::Result<Settings> {
        let file = fs::File::open(path)?;
        let settings: Settings = serde_yaml::from_reader(file)?;
        Ok(settings)
    }
}
