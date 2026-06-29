use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use std::fs;
use utoipa::ToSchema;

use crate::models::AssemblyJob;
use crate::error::AppError;
use crate::auth::AuthenticatedUser;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct AssembleRequest {
    pub code: String,
    pub title: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct AssembleResponse {
    pub job_id: String,
    pub intermediate: String,
    pub pass1: String,
    pub symb_table: String,
    pub lit_table: String,
    pub object_program: String,
}

#[utoipa::path(
    post,
    path = "/assemble",
    request_body = AssembleRequest,
    responses(
        (status = 200, description = "Assembly completed successfully", body = AssembleResponse)
    ),
    tag = "Assembly"
)]
pub async fn assemble(
    pool: web::Data<PgPool>,
    req: web::Json<AssembleRequest>,
    user: Option<AuthenticatedUser>,
) -> Result<impl Responder, AppError> {
    let job_id = Uuid::new_v4().to_string();
    let job_dir = format!("jobs/{}", job_id);
    let input_dir = format!("{}/input", job_dir);
    let output_dir = format!("{}/output", job_dir);
    
    // create directories
    fs::create_dir_all(&input_dir)
        .map_err(|e| AppError::InternalError(format!("Failed to create input directory: {}", e)))?;
    fs::create_dir_all(&output_dir)
        .map_err(|e| AppError::InternalError(format!("Failed to create output directory: {}", e)))?;
    
    // write input file
    let input_path = format!("{}/in.txt", input_dir);
    fs::write(&input_path, &req.code)
        .map_err(|e| AppError::InternalError(format!("Failed to write input file: {}", e)))?;
    
    // run assembler 
    let mut pass1_engine = crate::pass1::Pass1::new();
    let _ = pass1_engine.process_file(&input_path)
        .map_err(|e| AppError::BadRequest(format!("Failed to process source file: {}", e)))?;
    
    pass1_engine.pass1_generator(&output_dir)
        .map_err(|e| AppError::BadRequest(format!("Assembly Error (Pass 1): {}", e)))?;
        
    let intermediate_path = format!("{}/intermediate.txt", output_dir);
    let symbol_path = format!("{}/symbTable.txt", output_dir);
    let literal_path = format!("{}/litTable.txt", output_dir);
    let object_path = format!("{}/objectProgram.txt", output_dir);
    
    let mut pass2_engine = crate::pass2::Pass2::new();
    let _ = pass2_engine.pass2_generator(&intermediate_path, &symbol_path, &literal_path, &object_path)
        .map_err(|e| AppError::BadRequest(format!("Assembly Error (Pass 2): {}", e)))?;
        
    // read generated files to return them in the response and database
    let intermediate = fs::read_to_string(&intermediate_path)
        .map_err(|e| AppError::InternalError(format!("Failed to read intermediate file: {}", e)))?;
        
    let pass1 = fs::read_to_string(&intermediate_path)
        .map_err(|e| AppError::InternalError(format!("Failed to read intermediate file: {}", e)))?;
        
    let symb_table = fs::read_to_string(&symbol_path)
        .map_err(|e| AppError::InternalError(format!("Failed to read symbol table file: {}", e)))?;
        
    let lit_table = fs::read_to_string(&literal_path)
        .map_err(|e| AppError::InternalError(format!("Failed to read literal table file: {}", e)))?;
        
    let object_program = fs::read_to_string(&object_path)
        .map_err(|e| AppError::InternalError(format!("Failed to read object program file: {}", e)))?;
        
    // write pass1.txt for compatibility / backup
    fs::write(format!("{}/pass1.txt", output_dir), &pass1)
        .map_err(|e| AppError::InternalError(format!("Failed to write pass1 backup file: {}", e)))?;
    
    // save to database if authenticated
    if let Some(user) = user {
        let job_uuid = Uuid::parse_str(&job_id)
            .map_err(|e| AppError::InternalError(format!("Invalid job ID: {}", e)))?;
        
        let title = req.title.clone().unwrap_or_else(|| "Untitled Assembly".to_string());
        let now = Utc::now();
        
        sqlx::query(
            "INSERT INTO assembly_jobs (id, user_id, title, code, intermediate, pass1, symb_table, lit_table, object_program, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(job_uuid)
        .bind(user.user_id)
        .bind(&title)
        .bind(&req.code)
        .bind(&intermediate)
        .bind(&pass1)
        .bind(&symb_table)
        .bind(&lit_table)
        .bind(&object_program)
        .bind(now)
        .execute(pool.get_ref())
        .await?;
    }
    
    // clean up temporary job files from disk
    let _ = fs::remove_dir_all(&job_dir);

    let response = AssembleResponse {
        job_id,
        intermediate,
        pass1,
        symb_table,
        lit_table,
        object_program,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "User's assembly job history", body = Vec<AssemblyJob>),
        (status = 401, description = "Unauthorized - invalid or missing token")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Assembly"
)]
pub async fn get_history(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    let jobs = sqlx::query_as::<_, AssemblyJob>(
        "SELECT * FROM assembly_jobs WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user.user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(jobs))
}

#[utoipa::path(
    get,
    path = "/history/{id}",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job details", body = AssemblyJob),
        (status = 401, description = "Unauthorized - invalid or missing token"),
        (status = 404, description = "Job not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Assembly"
)]
pub async fn get_job(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let job_id = path.into_inner();
    let job_uuid = Uuid::parse_str(&job_id)
        .map_err(|_| AppError::BadRequest("Invalid job ID".to_string()))?;

    let job = sqlx::query_as::<_, AssemblyJob>(
        "SELECT * FROM assembly_jobs WHERE id = $1 AND user_id = $2"
    )
    .bind(job_uuid)
    .bind(user.user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    delete,
    path = "/history/{id}",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing token"),
        (status = 404, description = "Job not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Assembly"
)]
pub async fn delete_job(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let job_id = path.into_inner();
    let job_uuid = Uuid::parse_str(&job_id)
        .map_err(|_| AppError::BadRequest("Invalid job ID".to_string()))?;

    // verify user owns the job
    let _job = sqlx::query_as::<_, AssemblyJob>(
        "SELECT * FROM assembly_jobs WHERE id = $1 AND user_id = $2"
    )
    .bind(job_uuid)
    .bind(user.user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

    // delete from database
    sqlx::query("DELETE FROM assembly_jobs WHERE id = $1")
        .bind(job_uuid)
        .execute(pool.get_ref())
        .await?;

    // delete job directory from filesystem
    let job_dir = format!("jobs/{}", job_id);
    if fs::exists(&job_dir).unwrap_or(false) {
        fs::remove_dir_all(&job_dir)
            .map_err(|e| AppError::InternalError(format!("Failed to delete job directory: {}", e)))?;
    }

    tracing::info!("Deleted resources: Assembly job ID={} deleted by User ID={}", job_uuid, user.user_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Job deleted successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/history",
    responses(
        (status = 200, description = "All jobs deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing token")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Assembly"
)]
pub async fn delete_all_jobs(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, AppError> {
    // get all job IDs for this user to delete their directories
    let jobs = sqlx::query_as::<_, AssemblyJob>(
        "SELECT * FROM assembly_jobs WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_all(pool.get_ref())
    .await?;

    // delete all job directories from filesystem
    for job in &jobs {
        let job_dir = format!("jobs/{}", job.id);
        if fs::exists(&job_dir).unwrap_or(false) {
            let _ = fs::remove_dir_all(&job_dir);
        }
    }

    // delete all jobs from database
    sqlx::query("DELETE FROM assembly_jobs WHERE user_id = $1")
        .bind(user.user_id)
        .execute(pool.get_ref())
        .await?;

    tracing::info!("Deleted resources: All assembly jobs deleted by User ID={}", user.user_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "All jobs deleted successfully"
    })))
}
