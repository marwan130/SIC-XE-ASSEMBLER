# SIC-XE Assembler

A web-based SIC/XE assembler, featuring user authentication, assembly history, and OAuth integration.

## Quick Links

- [Backend Documentation](docs/BACKEND.md) - Rust/Actix-web server setup and API documentation
- [Frontend Documentation](docs/FRONTEND.md) - React/TypeScript frontend setup and development
- [Assembler Documentation](docs/ASSEMBLER.md) - SIC/XE assembly language implementation details
- [Docker Deployment](docker/README.md) - Docker and Railway deployment guide

## Overview

This project consists of three main components:

1. **Backend** - Rust-based REST API with PostgreSQL database
2. **Frontend** - React TypeScript web application with retro arcade UI
3. **Assembler** - SIC/XE assembly language implementation

## Features

- **SIC/XE Assembly**: Assemble SIC/XE code with full pass-1 and pass-2 support
- **User Authentication**: Email/password registration and login
- **OAuth Integration**: Sign in with Google or GitHub
- **Assembly History**: Save and retrieve past assembly sessions
- **Retro UI**: Retro arcade inspired design with CRT effects and animations
- **Theme System**: Switch between Neon, Cyber, and Pink accent themes

## Tech Stack

### Backend
- **Rust** with Actix-web framework
- **PostgreSQL** database with SQLx
- **JWT** authentication
- **OAuth2** (Google & GitHub)
- **OpenAPI/Swagger** documentation

### Frontend
- **React 18** with TypeScript
- **Vite** build tool
- **Tailwind CSS** for styling
- **Custom hooks** for state management

## Quick Start

### Prerequisites

- **Rust** 1.70 or later
- **Node.js** 18 or later
- **PostgreSQL** 14 or later
- **npm** or **yarn**

### Installation

1. Clone the repository:
```bash
git clone https://github.com/marwan130/SIC-XE-ASSEMBLER.git
cd SIC-XE-ASSEMBLER
```

2. Set up the database:
```bash
psql -U postgres -c "CREATE DATABASE sic_xe;"
psql -U postgres -d sic_xe -f migrations/001_create_users.sql
psql -U postgres -d sic_xe -f migrations/002_create_assembly_jobs.sql
```

3. Configure environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Start the backend:
```bash
cargo run --bin server
```

5. Start the frontend:
```bash
cd assembler-ui
npm install
npm run dev
```

For detailed setup instructions, see the component documentation links above.

## Project Structure

```
SIC-XE-ASSEMBLER/
├── src/                    # Backend source code
├── migrations/             # Database migrations
├── assembler-ui/           # Frontend application
├── docs/                   # Component documentation
├── .env.example            # Environment variables template
├── Cargo.toml              # Rust dependencies
├── LICENSE                 # MIT License
└── README.md               # This file
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.