#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: &'static [u8],
    pub jwt_maxage: i64,
    pub password_salt: &'static [u8],
    pub port: u16,
}

impl Config {
    /// Read config from environment variables, set to default values if not set
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL")
            .or::<String>(Ok("mysql://root:root@localhost:3306/riot".into()))
            .unwrap();
        let jwt_secret =
            match std::env::var("JWT_SECRET_KEY").or::<String>(Ok("RiotSecret!".into())) {
                Ok(salt) => Box::leak(salt.into_bytes().into_boxed_slice()),
                Err(_) => "r1oTs4lt".as_bytes(),
            };
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .or::<String>(Ok("86400".into()))
            .unwrap();

        // Make it a static ref
        let password_salt = match std::env::var("PWDSALT").or::<String>(Ok("r1oTs4lt".into())) {
            Ok(salt) => Box::leak(salt.into_bytes().into_boxed_slice()),
            Err(_) => "r1oTs4lt".as_bytes(),
        };

        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 8000,
            password_salt,
        }
    }
}
