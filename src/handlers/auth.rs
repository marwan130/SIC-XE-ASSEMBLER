use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{User, CreateUserRequest, LoginRequest, AuthResponse};
use crate::auth::encode_token;
use crate::error::AppError;

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
    // check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("User with this email already exists".to_string()));
    }

    // hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?;

    // create user
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, provider, created_at)
         VALUES ($1, $2, $3, $4, 'local', $5)"
    )
    .bind(user_id)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .bind(now)
    .execute(pool.get_ref())
    .await?;

    let user = User {
        id: user_id,
        email: req.email.clone(),
        password_hash: Some(password_hash),
        name: req.name.clone(),
        avatar_url: None,
        provider: "local".to_string(),
        provider_id: None,
        created_at: now,
    };

    // generate JWT
    let token = encode_token(&user)
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))?;

    let response = AuthResponse {
        token,
        user: user.clone(),
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    // find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 AND provider = 'local'"
    )
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // verify password
    let is_valid = verify(&req.password, user.password_hash.as_ref().unwrap())
        .map_err(|e| AppError::InternalError(format!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::BadRequest("Invalid password".to_string()));
    }

    let token = encode_token(&user)
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))?;

    let response = AuthResponse {
        token,
        user,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn me(
    user: crate::auth::AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {
    let user_data = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(user_data))
}
