use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use sea_orm::ConnectOptions;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfiguration {
    pub debug: Option<bool>,
    pub api: ApiConfiguration,
    pub media: MediaConfiguration,
    pub datasource: DataSourceConfiguration,
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

#[derive(Clone, Debug, Deserialize)]
pub struct DataSourceConfiguration {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DataSourceConfiguration {
    pub fn get_connection_options(uri: &str) -> ConnectOptions {
        let mut options = ConnectOptions::new(uri.to_owned());
        options.max_connections(100).sqlx_logging(true);

        options
    }

    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.user, &self.password, &self.host, self.port, &self.database
        )
    }

    pub fn get_connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            &self.user, &self.password, &self.host, self.port
        )
    }
}

pub fn load_config() -> Result<AppConfiguration, ConfigError> {
    dotenv().ok();

    let configuration = Config::builder()
        .add_source(File::with_name("configuration/default.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("app").separator("_"))
        .build()
        .unwrap();

    // Deserialize the configuration into the AppConfig struct
    let app_config: AppConfiguration = configuration.try_deserialize()?;

    Ok(app_config)
}