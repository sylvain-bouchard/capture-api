use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfiguration {
    pub debug: Option<bool>,
    pub api: ApiConfiguration,
    pub media: MediaConfiguration,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct ApiConfiguration {
    pub local_ip: String,
    pub url: Option<String>,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct MediaConfiguration {
    pub enabled: bool,
    pub recording_duration: u16,
}

pub fn load_config() -> Result<AppConfiguration, ConfigError> {
    dotenv().ok();

    let configuration = Config::builder()
        .add_source(File::with_name("configuration/default.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("app"))
        .build()
        .unwrap();

    // Deserialize the configuration into the AppConfig struct
    let app_config: AppConfiguration = configuration.try_deserialize()?;

    Ok(app_config)
}