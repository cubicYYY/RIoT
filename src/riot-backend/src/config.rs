use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE_URL: String =
        std::env::var("DATABASE_URL").unwrap_or("mysql://root:root@localhost:3306/riot".into());
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET_KEY").unwrap_or("RiotSecret!".into());
    static ref JWT_MAXAGE: String = std::env::var("JWT_MAXAGE").unwrap_or("86400".into());
    static ref PASSWORD_SALT: String = std::env::var("PWDSALT").unwrap_or("r1oTs4lt".into());
    static ref HOST: String = std::env::var("RIOT_HOST").unwrap_or("127.0.0.1:8888".into());
    static ref EMAIL: String = std::env::var("RIOT_EMAIL").unwrap_or("rust_iot@126.com".into());
    static ref SMTP_HOST: String =
        std::env::var("RIOT_SMTP_HOST").unwrap_or("smtp.126.com".into());
    static ref SMTP_NAME: String = std::env::var("RIOT_SMTP_NAME").unwrap_or("rust_iot".into());
    static ref SMTP_PWD: String =
        std::env::var("RIOT_SMTP_PWD").unwrap_or("AAAAAAAAAAAAAAAA".into());
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: &'static str,
    pub jwt_secret: &'static [u8],
    pub jwt_maxage: i64,
    pub password_salt: &'static [u8],
    pub host: &'static str,
    pub email: &'static str,
    pub smtp_host: &'static str,
    pub smtp_name: &'static str,
    pub smtp_pwd: &'static str,
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
            email: &EMAIL,
            smtp_host: &SMTP_HOST,
            smtp_name: &SMTP_NAME,
            smtp_pwd: &SMTP_PWD,
        }
    }
}
