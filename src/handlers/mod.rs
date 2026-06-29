pub mod auth;
pub mod assembly;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SIC/XE Assembler",
        version = "0.1.0",
        description = "API for SIC/XE assembler with authentication and job management"
    ),
    paths(
        auth::register,
        auth::login,
        auth::me,
        auth::delete_account,
        auth::logout,
        auth::google_auth,
        auth::google_callback,
        auth::github_auth,
        auth::github_callback,
        assembly::assemble,
        assembly::get_history,
        assembly::get_job,
        assembly::delete_job,
        assembly::delete_all_jobs,
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Assembly", description = "Assembly job endpoints"),
    ),
    components(
        schemas(
            crate::models::User,
            crate::models::AssemblyJob,
            crate::models::CreateUserRequest,
            crate::models::LoginRequest,
            crate::models::AuthResponse,
            assembly::AssembleRequest,
            assembly::AssembleResponse,
        )
    )
)]
pub struct ApiDoc;

pub use auth::{register, login, me, delete_account, logout, google_auth, google_callback, github_auth, github_callback};
pub use assembly::{assemble, get_history, get_job, delete_job, delete_all_jobs};