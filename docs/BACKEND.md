# Backend Documentation

Rust-based REST API server for the SIC-XE Assembler project.

## Overview

The backend is built with Actix-web and provides:
- RESTful API for assembly operations
- JWT-based authentication
- OAuth2 integration (Google & GitHub)
- PostgreSQL database integration
- OpenAPI/Swagger documentation

## Tech Stack

- **Rust** 1.70+
- **Actix-web** - Web framework
- **SQLx** - Database toolkit
- **PostgreSQL** - Database
- **JWT** - Authentication
- **OAuth2** - Third-party authentication
- **Utoipa/SwaggerUI** - OpenAPI documentation

## Setup

### Prerequisites

- Rust 1.70 or later
- PostgreSQL 14 or later

### Installation

1. Install dependencies:
```bash
cargo build
```

2. Set up environment variables (see `.env.example`):
```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/sic_xe
BIND_ADDRESS=127.0.0.1:8080
JWT_SECRET=your-secret-key
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret
FRONTEND_URL=http://localhost:5173
API_URL=http://localhost:8080
```

3. Run database migrations:
```bash
psql -U postgres -d sic_xe -f migrations/001_create_users.sql
psql -U postgres -d sic_xe -f migrations/002_create_assembly_jobs.sql
```

4. Start the server:
```bash
cargo run --bin server
```

The server will start on `http://127.0.0.1:8080`

API documentation available at: `http://127.0.0.1:8080/swagger-ui`

## API Endpoints

### Authentication

#### POST /auth/register
Register a new user.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "User Name"
}
```

**Response:** `201 Created`
```json
{
  "token": "jwt-token",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "User Name",
    "avatar_url": null,
    "provider": "local",
    "provider_id": null,
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

#### POST /auth/login
Login with email and password.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:** `200 OK`
```json
{
  "token": "jwt-token",
  "user": { ... }
}
```

#### GET /auth/me
Get current user information.

**Headers:** `Authorization: Bearer <token>`

**Response:** `200 OK`
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "name": "User Name",
  "avatar_url": null,
  "provider": "local",
  "provider_id": null,
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### GET /auth/google
Initiate Google OAuth flow.

**Response:** `307 Redirect` to Google OAuth page

#### GET /auth/google/callback
Google OAuth callback.

**Query Parameters:** `code`, `state`

**Response:** `307 Redirect` to frontend with token

#### GET /auth/github
Initiate GitHub OAuth flow.

**Response:** `307 Redirect` to GitHub OAuth page

#### GET /auth/github/callback
GitHub OAuth callback.

**Query Parameters:** `code`, `state`

**Response:** `307 Redirect` to frontend with token

### Assembly

#### POST /assemble
Assemble SIC/XE code.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "code": "START 1000\nLDA ALPHA\n...",
  "title": "My Program"
}
```

**Response:** `200 OK`
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "title": "My Program",
  "code": "...",
  "intermediate": "...",
  "pass1": "...",
  "symb_table": "...",
  "lit_table": "...",
  "object_program": "...",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### GET /history
Get user's assembly history.

**Headers:** `Authorization: Bearer <token>`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "title": "My Program",
    "code": "...",
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

#### GET /history/{id}
Get specific assembly job.

**Headers:** `Authorization: Bearer <token>`

**Response:** `200 OK`
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "title": "My Program",
  "code": "...",
  "intermediate": "...",
  "pass1": "...",
  "symb_table": "...",
  "lit_table": "...",
  "object_program": "...",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### DELETE /history/{id}
Delete specific assembly job.

**Headers:** `Authorization: Bearer <token>`

**Response:** `200 OK`
```json
{
  "message": "Job deleted successfully"
}
```

#### DELETE /history
Delete all assembly jobs for the user.

**Headers:** `Authorization: Bearer <token>`

**Response:** `200 OK`
```json
{
  "message": "All jobs deleted successfully"
}
```

### Health

#### GET /health
Health check endpoint.

**Response:** `200 OK`
```json
{
  "status": "healthy",
  "service": "sic-xe-assembler"
}
```

## Security

### Authentication
- Passwords are hashed using bcrypt before storage
- JWT tokens are used for authentication
- Token expiration is configurable
- OAuth2 provides secure third-party authentication

### Database
- SQL injection prevention via parameterized queries (SQLx)
- User data isolation (user_id foreign key constraints)
- Cascade delete for orphaned records

### Input Validation
- Email format validation
- Password strength requirements (4-12 characters, letters and numbers)
- Name length validation
- Code validation in assembly operations

### OAuth Security
- Redirect URI validation
- Token exchange via secure HTTPS
- Scope limitation (openid, email, profile)

## Project Structure

```
src/
├── bin/
│   └── server.rs          # Main server entry point
├── handlers/
│   ├── auth.rs            # Authentication handlers
│   ├── assembly.rs        # Assembly handlers
│   └── mod.rs             # Handler exports
├── models.rs              # Data models
├── auth.rs                # JWT utilities
├── error.rs               # Error types
└── lib.rs                 # Library exports
```

## Development

### Running in Development

```bash
cargo run --bin server
```

### Building for Production

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Database Migrations

Migrations are located in the `migrations/` directory and should be run manually:

```bash
psql -U postgres -d sic_xe -f migrations/001_create_users.sql
psql -U postgres -d sic_xe -f migrations/002_create_assembly_jobs.sql
```

## Error Handling

All errors return a consistent JSON format:

```json
{
  "error": "Error message"
}
```

Common error codes:
- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Missing or invalid authentication
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string |
| `BIND_ADDRESS` | No | Server bind address (default: 127.0.0.1:8080) |
| `JWT_SECRET` | Yes | Secret key for JWT signing |
| `GOOGLE_CLIENT_ID` | Yes | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | Yes | Google OAuth client secret |
| `GITHUB_CLIENT_ID` | Yes | GitHub OAuth client ID |
| `GITHUB_CLIENT_SECRET` | Yes | GitHub OAuth client secret |
| `FRONTEND_URL` | Yes | Frontend URL for CORS |
| `API_URL` | Yes | Backend URL for OAuth callbacks |
