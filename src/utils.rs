use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_salt: [u8; 16],
    pub jwt_secret: String,
    pub jwt_expiration_secs: u32,
}

pub fn load_env() -> Config {
    dotenv().ok();

    let jwt_salt_str = std::env::var("JWT_SALT")
        .expect("JWT_SALT must be set in .env file and be exactly 16 characters");
    assert_eq!(jwt_salt_str.len(), 16, "JWT_SALT must be exactly 16 characters long");
    let mut jwt_salt = [0u8; 16];
    jwt_salt.copy_from_slice(jwt_salt_str.as_bytes());

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file");
    let jwt_expiration_secs = std::env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION must be set in .env file")
        .parse::<u32>()
        .expect("JWT_EXPIRATION must be a valid u32");

    Config {
        jwt_salt,
        jwt_secret,
        jwt_expiration_secs,
    }
}