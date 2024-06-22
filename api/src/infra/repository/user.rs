use sqlx::PgPool;

#[trait_variant::make]
pub trait UserRepository {}

impl UserRepository for PgPool {}
