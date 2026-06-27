use actix_web::{web, HttpResponse, Responder, HttpRequest};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use oauth2::{
    AuthorizationCode, CsrfToken, RedirectUrl, TokenResponse, basic::BasicClient,
    reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};

use crate::models::{User, CreateUserRequest, LoginRequest, AuthResponse};
use crate::auth::encode_token;
use crate::error::AppError;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request - user already exists or invalid data")
    ),
    tag = "Authentication"
)]
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

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
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

#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = User),
        (status = 401, description = "Unauthorized - invalid or missing token")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Authentication"
)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

fn google_client() -> Result<BasicClient, AppError> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| AppError::InternalError("GOOGLE_CLIENT_ID not set".to_string()))?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| AppError::InternalError("GOOGLE_CLIENT_SECRET not set".to_string()))?;
    let backend_url = std::env::var("API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let redirect_url = RedirectUrl::new(format!("{}/auth/google/callback", backend_url))
        .map_err(|e| AppError::InternalError(format!("Invalid redirect URL: {}", e)))?;

    Ok(BasicClient::new(
        oauth2::ClientId::new(client_id),
        Some(oauth2::ClientSecret::new(client_secret)),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid auth URL: {}", e)))?,
        Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid token URL: {}", e)))?),
    )
    .set_redirect_uri(redirect_url))
}

#[utoipa::path(
    get,
    path = "/auth/google",
    responses(
        (status = 307, description = "Redirects to Google OAuth authorization page")
    ),
    tag = "Authentication"
)]
pub async fn google_auth(_req: HttpRequest) -> Result<HttpResponse, AppError> {
    let client = google_client()?;
    
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("openid".to_string()))
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .url();

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url.as_str()))
        .finish())
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: String,
    picture: Option<String>,
}

#[utoipa::path(
    get,
    path = "/auth/google/callback",
    params(
        ("code" = String, Query, description = "OAuth authorization code"),
        ("state" = String, Query, description = "OAuth state parameter")
    ),
    responses(
        (status = 200, description = "OAuth callback successful", body = AuthResponse)
    ),
    tag = "Authentication"
)]
pub async fn google_callback(
    pool: web::Data<PgPool>,
    query: web::Query<OAuthCallbackQuery>,
) -> Result<impl Responder, AppError> {
    let client = google_client()?;
    
    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to exchange code: {}", e)))?;

    // Get user info from Google
    let http_client = reqwest::Client::new();
    let user_info: GoogleUserInfo = http_client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to get user info: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse user info: {}", e)))?;

    // Check if user exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE provider = 'google' AND provider_id = $1"
    )
    .bind(&user_info.id)
    .fetch_optional(pool.get_ref())
    .await?;

    let user = if let Some(user) = existing_user {
        user
    } else {
        // Create new user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, name, avatar_url, provider, provider_id, created_at)
             VALUES ($1, $2, $3, $4, 'google', $5, $6)"
        )
        .bind(user_id)
        .bind(&user_info.email)
        .bind(&user_info.name)
        .bind(&user_info.picture)
        .bind(&user_info.id)
        .bind(now)
        .execute(pool.get_ref())
        .await?;

        User {
            id: user_id,
            email: user_info.email,
            password_hash: None,
            name: user_info.name,
            avatar_url: user_info.picture,
            provider: "google".to_string(),
            provider_id: Some(user_info.id),
            created_at: now,
        }
    };

    // Generate JWT
    let jwt_token = encode_token(&user)
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))?;

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    // redirect to frontend with token in URL fragment
    let redirect_url = format!("{}/#token={}", frontend_url, jwt_token);

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}

fn github_client() -> Result<BasicClient, AppError> {
    let client_id = std::env::var("GITHUB_CLIENT_ID")
        .map_err(|_| AppError::InternalError("GITHUB_CLIENT_ID not set".to_string()))?;
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET")
        .map_err(|_| AppError::InternalError("GITHUB_CLIENT_SECRET not set".to_string()))?;
    let backend_url = std::env::var("API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let redirect_url = RedirectUrl::new(format!("{}/auth/github/callback", backend_url))
        .map_err(|e| AppError::InternalError(format!("Invalid redirect URL: {}", e)))?;

    Ok(BasicClient::new(
        oauth2::ClientId::new(client_id),
        Some(oauth2::ClientSecret::new(client_secret)),
        oauth2::AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid auth URL: {}", e)))?,
        Some(oauth2::TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .map_err(|e| AppError::InternalError(format!("Invalid token URL: {}", e)))?),
    )
    .set_redirect_uri(redirect_url))
}

#[utoipa::path(
    get,
    path = "/auth/github",
    responses(
        (status = 307, description = "Redirects to GitHub OAuth authorization page")
    ),
    tag = "Authentication"
)]
pub async fn github_auth(_req: HttpRequest) -> Result<HttpResponse, AppError> {
    let client = github_client()?;
    
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .url();

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url.as_str()))
        .finish())
}

#[derive(Debug, Deserialize)]
struct GitHubUserInfo {
    id: i64,
    email: Option<String>,
    login: String,
    avatar_url: Option<String>,
}

#[utoipa::path(
    get,
    path = "/auth/github/callback",
    params(
        ("code" = String, Query, description = "OAuth authorization code"),
        ("state" = String, Query, description = "OAuth state parameter")
    ),
    responses(
        (status = 200, description = "OAuth callback successful", body = AuthResponse)
    ),
    tag = "Authentication"
)]
pub async fn github_callback(
    pool: web::Data<PgPool>,
    query: web::Query<OAuthCallbackQuery>,
) -> Result<impl Responder, AppError> {
    let client = github_client()?;
    
    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to exchange code: {}", e)))?;

    // Get user info from GitHub
    let http_client = reqwest::Client::new();
    let user_info: GitHubUserInfo = http_client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header("User-Agent", "sic-xe-assembler")
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to get user info: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse user info: {}", e)))?;

    let provider_id = user_info.id.to_string();
    let email = user_info.email.unwrap_or_else(|| format!("{}@github.local", user_info.login));

    // Check if user exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE provider = 'github' AND provider_id = $1"
    )
    .bind(&provider_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let user = if let Some(user) = existing_user {
        user
    } else {
        // Create new user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, name, avatar_url, provider, provider_id, created_at)
             VALUES ($1, $2, $3, $4, 'github', $5, $6)"
        )
        .bind(user_id)
        .bind(&email)
        .bind(&user_info.login)
        .bind(&user_info.avatar_url)
        .bind(&provider_id)
        .bind(now)
        .execute(pool.get_ref())
        .await?;

        User {
            id: user_id,
            email,
            password_hash: None,
            name: user_info.login,
            avatar_url: user_info.avatar_url,
            provider: "github".to_string(),
            provider_id: Some(provider_id),
            created_at: now,
        }
    };

    // Generate JWT
    let jwt_token = encode_token(&user)
        .map_err(|e| AppError::InternalError(format!("Failed to generate token: {}", e)))?;

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    // redirect to frontend with token in URL fragment
    let redirect_url = format!("{}/#token={}", frontend_url, jwt_token);

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}
