use actix_web::{dev::Payload, error::ErrorUnauthorized, FromRequest, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: usize, // Expiration time
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // extract authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // remove "Bearer " prefix
                
                let jwt_secret = std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set");

                match decode_token(token, &jwt_secret) {
                    Ok(claims) => {
                        let user_id = match Uuid::parse_str(&claims.sub) {
                            Ok(id) => id,
                            Err(_) => return ready(Err(ErrorUnauthorized("Invalid user ID in token"))),
                        };
                        
                        return ready(Ok(AuthenticatedUser {
                            user_id,
                            email: claims.email,
                        }));
                    }
                    Err(e) => {
                        tracing::warn!("JWT decode error: {}", e);
                        return ready(Err(ErrorUnauthorized("Invalid token")));
                    }
                }
            }
        }

        ready(Err(ErrorUnauthorized("Missing or invalid authorization header")))
    }
}

pub fn encode_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}