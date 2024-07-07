use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(serde::Deserialize)]
pub struct Configs {
    // DB
    pub database_url: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_cookie_key: String,

    // API Server
    pub port: u16,
    pub host: String,
}

pub static CONFIG: Lazy<Configs> = Lazy::new(|| {
    dotenv().ok();

    Configs {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set."),

        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is not set."),
        jwt_cookie_key: env::var("JWT_COOKIE_KEY").expect("JWT_COOKIE_KEY is not set."),

        port: env::var("PORT")
            .unwrap_or(String::from("8080"))
            .parse()
            .expect("PORT must be a valid number"),
        host: env::var("HOST").unwrap_or(String::from("127.0.0.1")),
    }
});
