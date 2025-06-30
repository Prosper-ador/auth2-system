use dotenvy::dotenv;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_salt: [u8; 16],
    pub jwt_secret: String,
    pub jwt_expiration_secs: u32,
}

pub fn load_env() -> Config {
    dotenv().ok();

    let jwt_salt_str = std::env::var("JWT_SALT")
        .unwrap_or_else(|_| "your-super-secret-salt-here-change-in-production".to_string());
    
    // Hash the salt string to get exactly 16 bytes
    let mut hasher = Sha256::new();
    hasher.update(jwt_salt_str.as_bytes());
    let hash_result = hasher.finalize();
    let mut jwt_salt = [0u8; 16];
    jwt_salt.copy_from_slice(&hash_result[..16]);

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-here-change-in-production".to_string());
    let jwt_expiration_secs = std::env::var("JWT_EXPIRATION_SECS")
        .unwrap_or_else(|_| "86400".to_string())
        .parse::<u32>()
        .unwrap_or(86400);

    Config {
        jwt_salt,
        jwt_secret,
        jwt_expiration_secs,
    }
}

pub fn is_valid_email(email: &str) -> bool {
    // Simple regex, use a crate like "validator" for production
    email.contains('@') && email.contains('.')
}