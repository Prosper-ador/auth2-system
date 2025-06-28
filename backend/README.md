# Auth API Backend

A secure authentication and authorization API built with Rust, Axum, and JWT tokens.

## Features

- 🔐 JWT-based authentication
- 👥 Role-based access control (Admin/User)
- 📝 OpenAPI/Swagger documentation
- 🔒 Password hashing with bcrypt
- 🌐 CORS support
- 📊 Structured logging

## Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

## Quick Start

### 1. Install Dependencies

```bash
cd backend
cargo build
```

### 2. Environment Setup

Create a `.env` file in the backend directory:

```bash
JWT_SECRET=your-super-secret-jwt-key-here
JWT_SALT=your-super-secret-salt-here
JWT_EXPIRATION_SECS=86400
```

### 3. Run the Server

```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Documentation

Once the server is running, you can access:

- **Swagger UI**: http://localhost:3000/swagger-ui
- **OpenAPI Spec**: http://localhost:3000/api-docs/openapi.json

## API Endpoints

### Public Endpoints (No Authentication Required)

- `POST /login` - User login
- `POST /register` - User registration

### Protected Endpoints (Bearer Token Required)

- `GET /user/profile` - Get user profile
- `GET /admin/dashboard` - Admin dashboard (Admin role required)
- `POST /admin/register` - Register new admin (Admin role required)

## Development

### Project Structure

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── middleware/      # JWT authentication middleware
│   ├── models/          # Data models and schemas
│   ├── routes/          # API route handlers
│   └── utils.rs         # Utility functions
├── Cargo.toml           # Rust dependencies
├── openapi.json         # OpenAPI specification
└── openapitools.json    # OpenAPI tools configuration
```

### Adding New Routes

1. Create your route handler in `src/routes/`
2. Add the route to the router in `src/main.rs`
3. Update OpenAPI documentation if needed

### Testing

```bash
cargo test
```

## Deployment

### Docker Deployment

1. Build the Docker image:
```bash
docker build -t auth-api .
```

2. Run the container:
```bash
docker run -p 3000:3000 --env-file .env auth-api
```

### Production Considerations

- Use strong, unique JWT secrets
- Set up proper CORS configuration
- Use HTTPS in production
- Implement rate limiting
- Add database persistence
- Set up monitoring and logging

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `JWT_SECRET` | Secret key for JWT signing | Required |
| `JWT_SALT` | Salt for password hashing | Required |
| `JWT_EXPIRATION_SECS` | JWT token expiration time in seconds | 86400 (24 hours) |

## Security

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours by default
- Role-based access control implemented
- CORS configured for security

## License

MIT License 