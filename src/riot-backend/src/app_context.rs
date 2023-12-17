use crate::config::CONFIG;
use crate::db::DBClient;
use crate::utils::email::send_email_smtp;
use crate::utils::jwt::generate_token;
use actix_web::cookie::{self, Cookie};

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::PoolConfig;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use log::info;
use moka::future::Cache;

#[derive(Clone)]
pub struct AppState {
    pub env: &'static CONFIG,
    pub db: DBClient,
    // TODO: Move caches out of App State
    /// Access control. k: IP/Email
    pub rate_limit: Cache<String, ()>,
    /// For email verification
    pub one_time_code: Cache<String, u64>, // TODO: This should be moved to somewhere like Redis
}

// User ops
impl AppState {
    pub fn get_jwt_cookie(&self, uid: u64) -> Cookie {
        let jwt_token = generate_token(
            &uid.to_string(),
            self.env.jwt.secret.as_bytes(),
            self.env.jwt.maxage,
        )
        .unwrap();
        Cookie::build("token", jwt_token)
            .path("/")
            .max_age(cookie::time::Duration::new(self.env.jwt.maxage, 0))
            .http_only(true)
            .finish()
    }
    pub async fn send_verify_mail(
        &self,
        user_email: &str,
        link: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Sending verification email to {}", user_email);
        let smtp_credentials = Credentials::new(
            self.env.email.smtp_username.to_string(),
            self.env.email.smtp_password.to_string(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
            &self.env.email.smtp_relay_server,
        )?
        // Add credentials for authentication
        .credentials(smtp_credentials)
        // Connection pool settings
        .pool_config(PoolConfig::new().max_size(20))
        .build();

        send_email_smtp(
            &mailer,
            &format!("RIoT <{}>", self.env.email.addr),
            &format!("<{}>", user_email),
            "RIoT Verification",
            format!(include_str!("email.tplt"), link = link),
        )
        .await
    }
}
