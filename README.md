# Axum REST API Boilerplate

A production-ready boilerplate for building REST APIs with Rust using the Axum web framework, PostgreSQL database, and Docker containerization.

## Features

- 🚀 **Axum Framework**: Fast, ergonomic web framework built on Tokio
- 🐘 **PostgreSQL Integration**: Database connection pooling with SQLx
- 🐳 **Docker Support**: Multi-stage Dockerfile and Docker Compose setup
- 🔧 **Configuration Management**: Environment-based configuration
- 📊 **Health Checks**: Built-in health check endpoint with database status
- 🛡️ **Error Handling**: Structured error responses and middleware
- 📝 **Logging**: Structured logging with tracing
- 🔄 **Hot Reload**: Development setup with volume mounting

## Quick Start

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://rustup.rs/) (optional, for local development)

### Running with Docker Compose (Recommended)

1. Create environment file (see [Environment Variables](#environment-variables)):
```bash
cp .env.example .env
```

2. Start the application:
```bash
docker-compose -f docker-compose.dev.yml up --build
```

The API will be available at `http://localhost:3000`

### Local Development Setup

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Start PostgreSQL (using Docker):
```bash
docker run --name postgres-dev \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=axum_db \
  -p 5432:5432 \
  -d postgres:15-alpine
```

3. Set environment variables:
```bash
export DATABASE_URL="postgresql://postgres:password@localhost:5432/axum_db"
export SERVER_HOST="127.0.0.1"
export SERVER_PORT="3000"
```

4. Run the application:
```bash
cargo run
```

## API Documentation

### Base URL
```
http://localhost:3000
```

### Endpoints

#### Health Check
Check the application and database status.

**GET** `/health`

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2024-01-08T10:30:00Z",
  "database": "connected"
}
```

**Status Codes:**
- `200 OK`: Service is healthy
- `500 Internal Server Error`: Service or database issues

#### Example Error Endpoint
Demonstrates error handling (for testing purposes).

**GET** `/error`

**Response:**
```json
{
  "error": "ValidationError",
  "message": "This is an example validation error"
}
```

**Status Codes:**
- `400 Bad Request`: Validation error example

## Environment Variables

The application uses the following environment variables:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `SERVER_HOST` | Server bind address | `0.0.0.0` | No |
| `SERVER_PORT` | Server port | `3000` | No |
| `RUST_LOG` | Logging level | `info` | No |

### Example Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/axum_db
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=debug
```

## Project Structure

```
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library root module
│   ├── config/              # Configuration management
│   │   └── mod.rs
│   ├── database/            # Database connection and pool
│   │   └── mod.rs
│   ├── handlers/            # Request handlers
│   │   └── mod.rs
│   ├── routes/              # Route definitions
│   │   └── mod.rs
│   ├── models/              # Data models
│   │   └── mod.rs
│   ├── errors.rs            # Error types and handling
│   └── middleware.rs        # Custom middleware
├── docker/
│   └── postgres/
│       └── init/            # Database initialization scripts
│           └── 01-init.sql
├── Dockerfile               # Multi-stage Docker build
├── docker-compose.dev.yml   # Development environment
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## Development

### Building the Project

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Database Migrations

This boilerplate includes SQLx for database operations. To add migrations:

1. Install SQLx CLI:
```bash
cargo install sqlx-cli
```

2. Create a migration:
```bash
sqlx migrate add create_users_table
```

3. Run migrations:
```bash
sqlx migrate run
```

### Docker Commands

```bash
# Build the Docker image
docker build -t axum-api .

# Run the container
docker run -p 3000:3000 --env-file .env axum-api

# View logs
docker-compose -f docker-compose.dev.yml logs -f api

# Stop services
docker-compose -f docker-compose.dev.yml down
```

## Production Deployment

### Docker Image

The Dockerfile uses a multi-stage build to create an optimized production image:

1. **Builder stage**: Compiles the Rust application
2. **Runtime stage**: Minimal Alpine Linux image with the compiled binary

### Environment Setup

For production deployment:

1. Set appropriate environment variables
2. Use a production PostgreSQL instance
3. Configure proper logging levels
4. Set up health check monitoring
5. Configure reverse proxy (nginx, traefik, etc.)

### Security Considerations

- Use strong database passwords
- Configure CORS appropriately for your domain
- Set up TLS/SSL termination
- Implement rate limiting
- Use secrets management for sensitive configuration

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

### Common Issues

**Database Connection Failed**
- Ensure PostgreSQL is running and accessible
- Check the `DATABASE_URL` environment variable
- Verify database credentials and network connectivity

**Port Already in Use**
- Change the `SERVER_PORT` environment variable
- Kill the process using the port: `lsof -ti:3000 | xargs kill`

**Docker Build Issues**
- Clear Docker cache: `docker system prune -a`
- Ensure Docker has enough memory allocated
- Check Dockerfile syntax and dependencies

### Getting Help

- Check the [Issues](https://github.com/your-repo/issues) page
- Review the application logs: `docker-compose logs api`
- Enable debug logging: `RUST_LOG=debug`