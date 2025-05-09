use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Duration, Utc};
use actix_web::{error::ErrorUnauthorized, Error};

use crate::utils::models::User;

/// Struct to represent the claims embedded in the JWT.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,  // email
    pub role: String, // role (admin or user)
    pub exp: usize,   // expiration timestamp
}

/// Creates a JWT token for the provided user.
pub fn create_jwt(user: &User) -> Result<String, Error> {
    // Set expiration time (1 hour from now)
    let claims = Claims {
        sub: user.email.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    // Get the secret key from the environment variable
    let key = env::var("JWT_SECRET").map_err(|_| ErrorUnauthorized("JWT_SECRET not set"))?;
    let encoding_key = EncodingKey::from_secret(key.as_bytes());

    // Encode the JWT with the claims and secret key
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(format!("JWT creation failed: {}", e)))
}

/// Validates the provided JWT and returns the claims if valid.
pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    // Get the secret key from the environment variable
    let key = env::var("JWT_SECRET").map_err(|_| ErrorUnauthorized("JWT_SECRET not set"))?;
    let decoding_key = DecodingKey::from_secret(key.as_bytes());

    // Configure validation settings for token
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true; // Ensure the token is not expired

    // Decode the JWT and validate it
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| ErrorUnauthorized(format!("Token validation error: {}", e)))?;

    // Return the claims from the token if it's valid
    Ok(token_data.claims)
}
