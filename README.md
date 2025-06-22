# Auth API

A simple, secure authentication API built with [Axum](https://github.com/tokio-rs/axum) and [Rust](https://www.rust-lang.org/).  
This project demonstrates user registration and login with JWT-based authentication, password hashing, and OpenAPI documentation via [utoipa](https://github.com/juhaku/utoipa).

---

## Features

- **User Registration**: Register new users with hashed passwords.
- **User Login**: Authenticate users and issue JWT tokens.
- **JWT Authentication**: Secure protected routes using middleware.
- **OpenAPI Docs**: Auto-generated API documentation with utoipa.
- **In-memory User Store**: Simple storage for demonstration (swap for a database in production).
- **Password Hashing**: Secure password storage using bcrypt.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/)

### Clone the Repository

```sh
git clone https://github.com/your-username/auth_api.git
cd auth_api
```

### Set Environment Variables

Set the JWT secret key (required for signing tokens):

```sh
export JWT_SECRET="your_super_secret_key"
```

### Build and Run

```sh
cargo run
```

The API will start (by default) on `http://localhost:3000` or your configured port.

---

## API Endpoints

### Register

- **POST** `/register`
- **Request Body**:
    ```json
    {
      "email": "user@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "password": "yourpassword",
      "confirm_password": "yourpassword"
    }
    ```
- **Response**:
    - `201 Created`  
      ```json
      {
        "message": "User registered successfully",
        "token": "<jwt_token>"
      }
      ```
    - `400 Bad Request`  
      ```json
      { "error": "User already exists" }
      ```

### Login

- **POST** `/login`
- **Request Body**:
    ```json
    {
      "email": "user@example.com",
      "password": "yourpassword"
    }
    ```
- **Response**:
    - `200 OK`  
      ```json
      {
        "message": "Login successful",
        "token": "<jwt_token>"
      }
      ```
    - `401 Unauthorized`  
      ```json
      { "error": "Invalid credentials" }
      ```

### Protected Routes

To access protected endpoints, include the JWT token in the `Authorization` header:

```
Authorization: Bearer <jwt_token>
```

---

## API Documentation

OpenAPI docs are generated with utoipa.  
You can expose and view the Swagger UI by adding a route for it in your Axum app (see utoipa documentation for integration).

---

## Security Notes

- **Passwords** are hashed using bcrypt before storage.
- **JWT tokens** are signed with your `JWT_SECRET` environment variable.
- **User data** is stored in-memory for demonstration.  
  **For production, use a persistent database.**

---

## Development

- Format code: `cargo fmt`
- Run tests: `cargo test`
- Lint: `cargo clippy`

---

## License

This project is licensed under the MIT License.

---

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

---

## Contact

For questions or support, please open an issue or contact