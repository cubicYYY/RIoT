use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// subject (the user)
    pub sub: String,
    /// issued at time
    pub iat: usize,
    /// expiration time
    pub exp: usize,
}

pub fn generate_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
    let claims = JwtClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn parse_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    let decoded = decode::<JwtClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );
    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), 401)),
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_create_and_decoded_valid_token() {
        let user_id = Uuid::new_v4().to_string();
        let secret = b"RiotSecret!";

        let token = generate_token(&user_id, secret, 60).unwrap();
        let decoded_user_id = parse_token(&token, secret).unwrap();
        println!("{:?}", token);
        assert_eq!(decoded_user_id, user_id);
    }
}
