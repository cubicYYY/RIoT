use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE_URL: String =
        std::env::var("DATABASE_URL").unwrap_or("mysql://root:root@localhost:3306/riot".into());
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET_KEY").unwrap_or("RiotSecret!".into());
    static ref JWT_MAXAGE: String = std::env::var("JWT_MAXAGE").unwrap_or("86400".into());
    static ref PASSWORD_SALT: String = std::env::var("PWDSALT").unwrap_or("r1oTs4lt".into());
    static ref HOST: String = std::env::var("RIOT_HOST").unwrap_or("127.0.0.1:8888".into());
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: &'static str,
    pub jwt_secret: &'static [u8],
    pub jwt_maxage: i64,
    pub password_salt: &'static [u8],
    pub host: &'static str,
}

impl Config {
    /// Read config from environment variables, set to default values if not set
    pub fn init() -> Config {
        Config {
            database_url: &DATABASE_URL,
            jwt_secret: &JWT_SECRET.as_bytes(),
            jwt_maxage: JWT_MAXAGE.parse::<i64>().unwrap(),
            password_salt: &PASSWORD_SALT.as_bytes(),
            host: &HOST,
        }
    }
}
