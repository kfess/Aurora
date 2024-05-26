use anyhow::Result;
use sqlx::PgPool;

use crate::domain::vo::technique_tag::TechnicalTag;

pub struct TechnicalTagRepository {
    pool: PgPool,
}

pub trait TechnicalTagRepositoryTrait {
    async fn get_tags(&self, algo_id: Option<&str>) -> Result<Vec<TechnicalTag>>;
    async fn create_tag(&self, en_name: &str, ja_name: &str, algorithm_id: &str) -> Result<()>;
}

impl TechnicalTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TechnicalTagRepositoryTrait for TechnicalTagRepository {
    /// Get all tags or tags for a specific algorithm
    ///
    /// If `algo_id` is provided, only tags for that algorithm will be returned.
    /// Otherwise, all tags will be returned.
    async fn get_tags(&self, algo_id: Option<&str>) -> Result<Vec<TechnicalTag>> {
        let tags = match algo_id {
            Some(algo_id) => {
                let query = r#"
                    SELECT t.id, t.en_name, t.ja_name, a.name as algorithm_name
                    FROM technical_tags t
                    JOIN algorithms a ON t.algorithm_id = a.id
                    WHERE a.id = $1
                "#;

                let tags = sqlx::query_as::<_, TechnicalTag>(&query)
                    .bind(algo_id)
                    .fetch_all(&self.pool)
                    .await?;

                tags
            }
            None => {
                let query = r#"
                    SELECT t.id, t.en_name, t.ja_name, a.name as algorithm_name
                    FROM technical_tags t
                    JOIN algorithms a ON t.algorithm_id = a.id
                "#;

                let tags = sqlx::query_as::<_, TechnicalTag>(&query)
                    .fetch_all(&self.pool)
                    .await?;

                tags
            }
        };

        for tag in &tags {
            println!("{:?}", tag);
        }

        Ok(tags)
    }

    /// Create a new tag
    ///
    /// Generally, tags should be complete within the Technical Tag database.
    /// However, it should be possible to add new tags for certain cases.
    async fn create_tag(&self, en_name: &str, ja_name: &str, algorithm_id: &str) -> Result<()> {
        let query = r#"
            INSERT INTO technical_tags (id, en_name, ja_name, algorithm_id)
            VALUES ($1, $2, $3, $4)
        "#;

        sqlx::query(&query)
            .bind(en_name)
            .bind(ja_name)
            .bind(algorithm_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
