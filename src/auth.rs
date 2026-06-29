use actix_web::{dev::Payload, error::ErrorUnauthorized, web, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub exp: usize, // Expiration time
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // extract authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(str::to_owned);

        let pool = req.app_data::<web::Data<PgPool>>().cloned();

        Box::pin(async move {
            let Some(auth_header) = auth_header else {
                return Err(ErrorUnauthorized("Missing or invalid authorization header"));
            };

            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..]; // remove "Bearer " prefix

                let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

                match decode_token(token, &jwt_secret) {
                    Ok(claims) => {
                        let user_id = match Uuid::parse_str(&claims.sub) {
                            Ok(id) => id,
                            Err(_) => return Err(ErrorUnauthorized("Invalid user ID in token")),
                        };

                        let Some(pool) = pool else {
                            return Err(ErrorUnauthorized("Authentication is unavailable"));
                        };

                        let user_exists = sqlx::query_scalar::<_, bool>(
                            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
                        )
                        .bind(user_id)
                        .fetch_one(pool.get_ref())
                        .await
                        .map_err(|e| {
                            tracing::warn!("Failed to verify authenticated user exists: {}", e);
                            ErrorUnauthorized("Invalid token")
                        })?;

                        if !user_exists {
                            return Err(ErrorUnauthorized("User account no longer exists"));
                        }

                        return Ok(AuthenticatedUser {
                            user_id,
                            username: claims.username,
                        });
                    }
                    Err(e) => {
                        tracing::warn!("JWT decode error: {}", e);
                        return Err(ErrorUnauthorized("Invalid token"));
                    }
                }
            }

            Err(ErrorUnauthorized("Missing or invalid authorization header"))
        })
    }
}

pub fn encode_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
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
