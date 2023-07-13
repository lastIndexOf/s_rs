use anyhow::Result;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
    pub postgres: PostgresSettings,
}

#[derive(Deserialize)]
pub struct PostgresSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

impl PostgresSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn connection_without_db_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings> {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("configuration"))
        .build()?;

    Ok(settings.try_deserialize()?)
}
