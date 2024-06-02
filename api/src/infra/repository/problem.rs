use anyhow::Result;
use sqlx::PgPool;

use crate::domain::problem::Problem;

pub trait ProblemRepositoryTrait {
    async fn get_all_problems(&self, platform: &str) -> Result<Vec<Problem>>;
    // async fn get_problem_by_id(&self, id: &str) -> Result<Problem>;
    async fn update_problems(&self, platform: &str, problems: &[Problem]) -> Result<()>;
}

impl ProblemRepositoryTrait for PgPool {
    async fn get_all_problems(&self, platform: &str) -> Result<Vec<Problem>> {
        // let platform = Platform::try_from(platform)?;
        // let query = r#"
        //     SELECT
        // "#;

        // let rows = sqlx::query(&query).fetch_all(self).await?;
        // let problems: Vec<Problem> = rows
        //     .into_iter()
        //     .map(|row| {
        //         Problem::new(
        //             id,
        //             contest_id,
        //             index,
        //             name,
        //             title,
        //             platform,
        //             raw_point,
        //             difficulty,
        //             is_experimental,
        //             tags,
        //             url,
        //             solver_count,
        //             submissions,
        //             success_rate,
        //         )
        //     })
        //     .collect();

        Ok(vec![])
    }

    // async fn get_problem_by_id(&self, id: &str) -> Result<Problem> {}

    async fn update_problems(&self, platform: &str, problems: &[Problem]) -> Result<()> {
        for p in problems {
            println!("{:?}", p);
        }

        Ok(())
    }
}
