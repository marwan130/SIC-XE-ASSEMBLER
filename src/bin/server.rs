use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "sic-xe-assembler"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    // Create database connection pool (optional for now)
    let pool = if let Ok(database_url) = std::env::var("DATABASE_URL") {
        println!("Connecting to database...");
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("Connected to database");
                Some(pool)
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                eprintln!("Continuing without database connection");
                None
            }
        }
    } else {
        println!("DATABASE_URL not set, running without database");
        None
    };
    
    println!("Starting server on {}", bind_address);
    
    if let Some(pool) = pool {
        HttpServer::new(move || {
            let cors = Cors::permissive();
            let pool = pool.clone();
            
            App::new()
                .app_data(web::Data::new(pool))
                .wrap(cors)
                .route("/health", web::get().to(health))
        })
        .bind(&bind_address)?
        .run()
        .await
    } else {
        HttpServer::new(|| {
            let cors = Cors::permissive();
            
            App::new()
                .wrap(cors)
                .route("/health", web::get().to(health))
        })
        .bind(&bind_address)?
        .run()
        .await
    }
}
