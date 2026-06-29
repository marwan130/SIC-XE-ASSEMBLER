# Docker Configuration

This directory contains Dockerfiles for containerized deployment of the SIC/XE Assembler project.

## Files

- `Dockerfile` - Backend API server (Rust/Actix-web)
- `Dockerfile.frontend` - Frontend web app (React/Vite with nginx)
- `Dockerfile.assembler` - Assembler CLI tool
- `docker-compose.yml` - Local development orchestration
- `nginx.conf` - Nginx configuration for frontend

## Local Development

```bash
cd docker
docker compose up -d
```

## Railway Deployment

For Railway deployment, you don't need docker-compose.yml. Railway will use the Dockerfiles directly.

### Required Environment Variables (Railway Dashboard)

**Backend:**
- `DATABASE_URL` - Provided automatically by Railway PostgreSQL
- `JWT_SECRET` - Generate with `openssl rand -base64 32`
- `BIND_ADDRESS` - Set to `0.0.0.0:8080`
- `FRONTEND_URL` - Your Railway frontend URL
- `API_URL` - Your Railway backend URL
- `DB_MAX_CONNECTIONS` - Recommended: 10
- `ENABLE_SWAGGER` - Set to `false` for production

**Frontend:**
- `VITE_API_URL` - Your Railway backend URL

**OAuth (Optional):**
- `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET`
- `GITHUB_CLIENT_ID` / `GITHUB_CLIENT_SECRET`

## Railway Service Setup

1. **Backend Service:**
   - Root directory: `..` (project root)
   - Dockerfile path: `docker/Dockerfile`
   - Start command: `./server`

2. **Frontend Service:**
   - Root directory: `..` (project root)
   - Dockerfile path: `docker/Dockerfile.frontend`
   - Build args: `VITE_API_URL=${API_URL}`
   - Start command: `nginx -g "daemon off;"`

3. **PostgreSQL Service:**
   - Use Railway's managed PostgreSQL
   - Railway will automatically set `DATABASE_URL`