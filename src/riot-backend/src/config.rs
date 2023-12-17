use config::Config as ConfigUtil;
use log::warn;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init());

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SiteConfig {
    /// Your site host, e.g. "http://myriot.com"
    pub host: String,
    pub password_salt: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EmailConfig {
    pub addr: String,
    pub smtp_relay_server: String,
    /// SMTP username
    pub smtp_username: String,
    /// SMTP password/code
    pub smtp_password: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MysqlConfig {
    pub username: String,
    pub password: String,
    /// MySQL host e.g. "127.0.0.1"
    pub host: String,
    /// MySQL port
    pub port: u16,
    pub database: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JwtConfig {
    pub maxage: i64,
    pub secret: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub riot: SiteConfig,
    pub email: EmailConfig,
    pub jwt: JwtConfig,
    pub mysql: MysqlConfig,
}

impl Config {
    /// Read config from environment variables, set to default values if not set
    pub fn init() -> Self {
        let config_arg = std::env::args().nth(1);
        let config_file = if let Some(config) = config_arg.as_ref() {
            println!("The config file is {}", config);
            config
        } else {
            warn!("Config file not specified. Fallback to `riot_config.toml` .");
            "riot_config.toml"
        };
        let settings = ConfigUtil::builder()
            .add_source(config::File::with_name(config_file))
            // .add_source(config::Environment::with_prefix("RIOT"))
            .build()
            .expect("Failed to build config");
        dbg!(settings.try_deserialize::<Config>().unwrap())
    }
}
