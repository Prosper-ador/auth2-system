use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_salt: String,
    pub jwt_secret: String,
    pub jwt_expiration_secs: u32,
}

pub fn load_env() -> Config {
    dotenv().ok();

    let jwt_salt = std::env::var("JWT_SALT")
        .unwrap_or_else(|_| {
            println!("JWT_SALT must be set in .env file");
            std::process::exit(1);
        });
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            println!("JWT_SECRET must be set in .env file");
            std::process::exit(1);
        });
    let jwt_expiration_secs = std::env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| {
            println!("JWT_EXPIRATION must be set in .env file");
            std::process::exit(1);
        })
        .parse::<u32>()
        .unwrap();

    return Config {
        jwt_salt,
        jwt_secret,
        jwt_expiration_secs,
    };
}