use anyhow::{Context, Result};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::domain::problem::Problem;

pub trait ProblemRepository {
    async fn get_all_problems(&self) -> Result<Vec<Problem>>;
    async fn get_problems_by_platform(&self, platform: &str) -> Result<Vec<Problem>>;
    async fn get_problem_by_id(&self, id: &str) -> Result<Problem>;
    async fn update_problems(&self, problems: &[Problem]) -> Result<()>;
}

impl ProblemRepository for PgPool {
    async fn get_all_problems(&self) -> Result<Vec<Problem>> {
        let problems = sqlx::query_as::<_, Problem>(
            r#"
                SELECT problems.id, problems.contest_id, problems.problem_index AS "index!", problems.name,
                    problems.title, problems.platform, problems.raw_point, problems.difficulty,
                    problems.is_experimental, problems.url, problems.solver_count,
                    problems.submissions, problems.success_rate
                FROM problems
                INNER JOIN problem_tags ON problems.id = problem_tags.problem_id
                "#
        )
        .fetch_all(self)
        .await
        .with_context(|| format!("Failed to fetch problems"))?;

        Ok(problems)
    }

    async fn get_problems_by_platform(&self, platform: &str) -> Result<Vec<Problem>> {
        Ok(vec![])
    }

    async fn get_problem_by_id(&self, id: &str) -> Result<Problem> {
        let problem = sqlx::query_as::<_, Problem>(
            r#"
                SELECT problems.id, problems.contest_id, problems.problem_index AS "index!", problems.name,
                    problems.title, problems.platform, problems.raw_point, problems.difficulty,
                    problems.is_experimental, problems.url, problems.solver_count,
                    problems.submissions, problems.success_rate
                FROM problems
                INNER JOIN problem_tags ON problems.id = problem_tags.problem_id
                WHERE problems.id = $1
                "#,
        )
        .bind(id)
        .fetch_one(self)
        .await
        .with_context(|| format!("Failed to fetch problem"))?;

        Ok(problem)
    }

    async fn update_problems(&self, problems: &[Problem]) -> Result<()> {
        let mut transaction = self.begin().await?;

        for chunk in problems.chunks(100) {
            let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
                r#"
                INSERT INTO problems (
                    id, contest_id, problem_index, name, title, platform,
                    raw_point, difficulty, is_experimental, url,
                    solver_count, submissions, success_rate
                )
                "#,
            );

            query_builder.push_values(chunk, |mut separated, problem| {
                separated
                    .push_bind(&problem.id)
                    .push_bind(&problem.contest_id)
                    .push_bind(problem.index.as_str())
                    .push_bind(&problem.name)
                    .push_bind(&problem.title)
                    .push_bind(String::from(problem.platform))
                    .push_bind(problem.raw_point)
                    .push_bind(problem.difficulty)
                    .push_bind(problem.is_experimental)
                    .push_bind(&problem.url)
                    .push_bind(problem.solver_count)
                    .push_bind(problem.submissions)
                    .push_bind(problem.success_rate);
            });

            query_builder.push(
                r#"
                ON CONFLICT (id) DO UPDATE SET
                    contest_id = EXCLUDED.contest_id,
                    problem_index = EXCLUDED.problem_index,
                    name = EXCLUDED.name,
                    title = EXCLUDED.title,
                    platform = EXCLUDED.platform,
                    raw_point = EXCLUDED.raw_point,
                    difficulty = EXCLUDED.difficulty,
                    is_experimental = EXCLUDED.is_experimental,
                    url = EXCLUDED.url,
                    solver_count = EXCLUDED.solver_count,
                    submissions = EXCLUDED.submissions,
                    success_rate = EXCLUDED.success_rate
                "#,
            );

            let query = query_builder.build();
            query
                .execute(&mut *transaction)
                .await
                .with_context(|| format!("Failed to execute query"))?;
        }

        // problem_tags の upsert

        transaction.commit().await?;

        Ok(())
    }
}
