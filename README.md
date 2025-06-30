Ran tool
Ran tool
Read file: frontend/package.json
Read file: backend/Cargo.toml
Read file: backend/README.md
Read file: frontend/README.md
Here is a **professional, unified README** for your fullstack authentication/authorization project, covering both backend (Rust/Axum) and frontend (React/TypeScript/Tailwind):

---

# Fullstack Auth API

A modern, secure authentication and authorization system with a Rust/Axum backend and a React/TypeScript frontend. Features JWT-based auth, role-based access control, beautiful UI, and OpenAPI integration.

---

## Features

- 🔐 JWT-based authentication (login, registration)
- 👥 Role-based access control (User/Admin)
- 📝 OpenAPI/Swagger documentation (auto-generated client)
- 💻 Modern React + TypeScript frontend (Vite, Tailwind, shadcn-ui)
- 🌗 Dark mode, animated UI, responsive design
- 🛡️ Secure password hashing (bcrypt)
- 🌐 CORS support, structured logging

---

## Project Structure

```
auth_api/
├── backend/      # Rust/Axum API server
├── frontend/     # React/TypeScript client
├── ts-client/    # Auto-generated TypeScript API client (from OpenAPI)
```

---

## Getting Started

### Prerequisites

- **Backend:** Rust 1.70+ ([Install Rust](https://rustup.rs/))
- **Frontend:** Node.js 18+ and npm or yarn

---

### 1. Backend Setup

```bash
cd backend
cargo build
```

Create a `.env` file in `backend/`:

```env
JWT_SECRET=your-super-secret-jwt-key-here
JWT_SALT=your-super-secret-salt-here
JWT_EXPIRATION_SECS=86400
```

Start the backend server:

```bash
cargo run
```

- API runs at: `http://localhost:3000`
- Swagger UI: `http://localhost:3000/swagger-ui`
- OpenAPI Spec: `http://localhost:3000/api-docs/openapi.json`

---

### 2. Frontend Setup

```bash
cd frontend
npm install   # or yarn install
npm run dev   # or yarn dev
```

- App runs at: `http://localhost:5173` (default Vite port)

---

### 3. API Client (TypeScript)

The frontend uses an auto-generated TypeScript client (`ts-client/`) based on the backend OpenAPI spec.  
If you update backend endpoints, regenerate the client:

```bash
./generate-client.sh
```

---

## Usage

- **Register** a new user or log in with:
  - **Admin:** `admin@example.com` / `password`
  - **User:** `user@example.com` / `password`
- **Admin Dashboard:** Only visible to admin users after login.
- **Profile:** View your user info and session status.
- **Dark Mode:** Toggle from the navbar.

---

## API Endpoints

- `POST /login` — User login
- `POST /register` — User registration
- `GET /user/profile` — Get user profile (auth required)
- `GET /admin/dashboard` — Admin dashboard (admin only)
- `POST /admin/register` — Register new admin (admin only)

See [Swagger UI](http://localhost:3000/swagger-ui) for full docs.

---

## Environment Variables

**Backend:**
| Variable             | Description                        | Default   |
|----------------------|------------------------------------|-----------|
| `JWT_SECRET`         | Secret key for JWT signing         | Required  |
| `JWT_SALT`           | Salt for password hashing          | Required  |
| `JWT_EXPIRATION_SECS`| JWT token expiration (in seconds)  | 86400     |

**Frontend:**
| Variable             | Description                        | Default   |
|----------------------|------------------------------------|-----------|
| `VITE_API_BASE_URL`  | Backend API base URL               | http://localhost:3000 |

---

## Deployment

- **Test locally** as above.
- **Frontend:** Deploy to Vercel, Netlify, or your preferred static host.
- **Backend:** Deploy to your server, Railway, or any platform supporting Rust.

---

## Security & Production

- Use strong, unique secrets in production.
- Enable HTTPS and proper CORS.
- Add persistent database (current setup is in-memory for demo).
- Monitor and log API activity.

---

## License

MIT License

---

**Questions?**  
Open an issue or contact the maintainer.

---

Let me know if you want this saved as your main `README.md` or want a split (backend/frontend) version!