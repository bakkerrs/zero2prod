use config::{Config, ConfigError};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}


#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_namme: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
   let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;

    Ok(Settings::try_from(settings))
}