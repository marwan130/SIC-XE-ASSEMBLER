use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use systems_project::handlers::{register, login, me, delete_account, logout, google_auth, google_callback, github_auth, github_callback, assemble, get_history, get_job, delete_job, delete_all_jobs, ApiDoc};

// simple IP-based key extractor for rate limiting
struct PeerIpKeyExtractor;

impl KeyExtractor for PeerIpKeyExtractor {
    type Key = String;

    fn extract(&self, req: &actix_web::HttpRequest) -> Result<Self::Key, actix_governor::GovernorError> {
        Ok(req
            .peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string()))
    }
}

async fn health() -> impl Responder {
    info!("Health check requested");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "sic-xe-assembler"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();
    
    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let enable_swagger = std::env::var("ENABLE_SWAGGER")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Create database connection pool 
    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    info!("Connected to database successfully");

    // Run database migrations
    info!("Admin action: Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    info!("Admin action: Database migrations completed successfully");

    info!("Admin action: Starting server on {}", bind_address);
    info!("CORS allowed origin: {}", frontend_url);
    
    // configure rate limiting for auth endpoints
    let auth_governor_config = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .key_extractor(PeerIpKeyExtractor)
        .finish()
        .unwrap();

    let openapi = ApiDoc::openapi();
    
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_origin(&frontend_url)
            .allowed_origin_fn(|origin, _req| {
                let origin_bytes = origin.as_bytes();
                origin_bytes.starts_with(b"http://localhost") ||
                origin_bytes.starts_with(b"http://127.0.0.1")
            })
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let pool = pool.clone();
        let auth_governor = auth_governor_config.clone();

        let mut app = App::new()
            .app_data(web::Data::new(pool))
            .wrap(cors)
            .route("/health", web::get().to(health))
            .route("/auth/me", web::get().to(me))
            .route("/auth/delete", web::delete().to(delete_account))
            .route("/auth/logout", web::post().to(logout))
            .route("/auth/google", web::get().to(google_auth))
            .route("/auth/google/callback", web::get().to(google_callback))
            .route("/auth/github", web::get().to(github_auth))
            .route("/auth/github/callback", web::get().to(github_callback))
            .route("/assemble", web::post().to(assemble))
            .route("/history", web::get().to(get_history))
            .route("/history", web::delete().to(delete_all_jobs))
            .route("/history/{id}", web::get().to(get_job))
            .route("/history/{id}", web::delete().to(delete_job));

        // apply rate limiting to auth endpoints
        app = app.service(
            web::scope("/auth")
                .wrap(Governor::new(&auth_governor))
                .service(web::resource("/register").route(web::post().to(register)))
                .service(web::resource("/login").route(web::post().to(login)))
        );

        if enable_swagger {
            app = app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            );
        }

        app
    })
    .bind(&bind_address)?
    .run()
    .await
}