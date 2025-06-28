# Fullstack Authentication System

A complete authentication and authorization system with a Rust/Axum backend and React/TypeScript frontend.

## 🏗️ Project Structure

```
auth_api/
├── backend/           # Rust/Axum API server
│   ├── src/          # Rust source code
│   ├── Cargo.toml    # Rust dependencies
│   ├── Dockerfile    # Docker configuration
│   ├── docker-compose.yml
│   └── README.md     # Backend documentation
├── frontend/         # React/TypeScript client
│   ├── src/          # React source code
│   ├── package.json  # Node.js dependencies
│   └── README.md     # Frontend documentation
├── ts-client/        # Generated TypeScript API client
└── README.md         # This file
```

## 🚀 Quick Start

### Backend Setup

1. **Navigate to backend directory:**
   ```bash
   cd backend
   ```

2. **Set up environment:**
   ```bash
   cp env.example .env
   # Edit .env with your JWT secrets
   ```

3. **Run with Docker (recommended):**
   ```bash
   ./deploy.sh
   ```

4. **Or run locally:**
   ```bash
   cargo run
   ```

The backend will be available at `http://localhost:3000`

### Frontend Setup

1. **Navigate to frontend directory:**
   ```bash
   cd frontend
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

The frontend will be available at `http://localhost:5173`

## 🔧 Features

### Backend (Rust/Axum)
- 🔐 JWT-based authentication
- 👥 Role-based access control (Admin/User)
- 📝 OpenAPI/Swagger documentation
- 🔒 Password hashing with bcrypt
- 🌐 CORS support
- 📊 Structured logging
- 🐳 Docker deployment ready

### Frontend (React/TypeScript)
- 🎨 Modern UI with Tailwind CSS
- 🔄 Type-safe API client
- 🛡️ Protected routes
- 📱 Responsive design
- 🔔 Toast notifications
- 🎯 Form validation

## 📚 API Documentation

Once the backend is running, access the API documentation:

- **Swagger UI**: http://localhost:3000/swagger-ui
- **OpenAPI Spec**: http://localhost:3000/api-docs/openapi.json

## 🔐 Authentication Flow

1. **Registration**: Users can register with email, password, and personal info
2. **Login**: Users authenticate with email/password and receive JWT token
3. **Protected Routes**: Frontend uses JWT token for authenticated requests
4. **Role-based Access**: Different endpoints require different user roles

## 🛠️ Development

### Backend Development
```bash
cd backend
cargo run
```

### Frontend Development
```bash
cd frontend
npm run dev
```

### Regenerate TypeScript Client
```bash
cd backend
cargo run
# In another terminal:
cd ts-client
npm run generate
```

## 🚀 Deployment

### Backend Deployment
The backend is containerized and ready for deployment:

```bash
cd backend
./deploy.sh
```

### Frontend Deployment
Build the frontend for production:

```bash
cd frontend
npm run build
```

## 📝 Environment Variables

### Backend (.env)
```bash
JWT_SECRET=your-super-secret-jwt-key
JWT_SALT=your-super-secret-salt
JWT_EXPIRATION_SECS=86400
```

## 🔒 Security Features

- JWT tokens with expiration
- Password hashing with bcrypt
- Role-based access control
- CORS configuration
- Input validation
- Secure headers

## 📄 License

MIT License