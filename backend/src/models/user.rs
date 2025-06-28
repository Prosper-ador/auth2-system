use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String, // Hashed in production
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse { // This is the public user info returned after login
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize,Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub message: String,
    pub token_type: String,
}
