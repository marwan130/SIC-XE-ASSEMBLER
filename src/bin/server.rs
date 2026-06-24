use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber;

async fn health() -> impl Responder {
    info!("Health check requested");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "sic-xe-assembler"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();
    
    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    
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
    info!("Starting server on {}", bind_address);
    info!("CORS allowed origin: {}", frontend_url);
    
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_origin(&frontend_url)
            .allowed_origin_fn(|origin, _req| {
                origin.as_bytes().starts_with(b"http://localhost") ||
                origin.as_bytes().starts_with(b"http://127.0.0.1")
            })
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let pool = pool.clone();
        
        App::new()
            .app_data(web::Data::new(pool))
            .wrap(cors)
            .route("/health", web::get().to(health))
    })
    .bind(&bind_address)?
    .run()
    .await
}