use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    exp: usize,
}

pub fn create_token(username: &str, secret_key: &[u8]) -> Result<String, Error> {
    let expiration = Utc::now() + Duration::minutes(5);
    let my_claims = Claims {
        username: username.to_owned(),
        exp: expiration.timestamp() as usize, //for testing purposes only 1 minute
    };
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_key),
    )
}

pub fn verify_token(token: &str, secret_key: &[u8]) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::default(),
    )
}
