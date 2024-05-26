use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TechnicalTag {
    pub id: String,
    pub en_name: String,
    pub ja_name: String,
    pub algorithm_name: String,
}
