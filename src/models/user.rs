use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String, // Hashed in production
    pub role: Role,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}