use anyhow::{Context, Result};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::domain::problem::Problem;

pub struct Condition<'a> {
    pub platform: Option<&'a str>,
    pub algo_id: Option<&'a str>,
    pub technical_tag_id: Option<&'a str>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub from_difficulty: Option<i32>,
    pub to_difficulty: Option<i32>,
}

impl Default for Condition<'_> {
    fn default() -> Self {
        Self {
            platform: Some("atcoder"),
            algo_id: None,
            technical_tag_id: None,
            page: None,
            page_size: None,
            from_difficulty: None,
            to_difficulty: None,
        }
    }
}

enum BindValue<'a> {
    Str(&'a str),
    I32(i32),
}

#[trait_variant::make]
pub trait ProblemRepository {
    async fn get_problems_by_condition(&self, condition: &Condition<'_>) -> Result<Vec<Problem>>;
    async fn get_problem_by_id(&self, id: &str) -> Result<Problem>;
    async fn update_problems(&self, problems: &[Problem]) -> Result<()>;
}

impl ProblemRepository for PgPool {
    async fn get_problems_by_condition(&self, condition: &Condition<'_>) -> Result<Vec<Problem>> {
        let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
            r#"
                SELECT
                    problems.id,
                    problems.contest_id,
                    problems.problem_index AS index,
                    problems.name,
                    problems.title,
                    problems.platform,
                    problems.raw_point,
                    problems.difficulty,
                    problems.is_experimental,
                    problems.url,
                    problems.solver_count,
                    problems.submissions,
                    problems.success_rate,
                    ARRAY_REMOVE (ARRAY_AGG (technical_tags.en_name), NULL) AS tags
                FROM
                    problems
                    LEFT JOIN problem_tags ON problems.id = problem_tags.problem_id
                    LEFT JOIN technical_tags ON problem_tags.technical_tag_id = technical_tags.id
                "#,
        );

        let mut conditions: Vec<(&str, BindValue)> = Vec::new();

        if let Some(platform) = condition.platform {
            conditions.push(("problems.platform = ", BindValue::Str(platform)));
        }

        if let Some(algo_id) = condition.algo_id {
            conditions.push(("technical_tags.algorithm_id = ", BindValue::Str(algo_id)));
        }

        if let Some(technical_tag_id) = condition.technical_tag_id {
            conditions.push((
                "problem_tags.technical_tag_id = ",
                BindValue::Str(technical_tag_id),
            ));
        }

        if !conditions.is_empty() {
            query_builder.push(" WHERE ");

            for (i, (column, value)) in conditions.iter().enumerate() {
                if i > 0 {
                    query_builder.push(" AND ");
                }
                match value {
                    BindValue::Str(v) => {
                        query_builder.push(column).push_bind(v);
                    }
                    BindValue::I32(v) => {
                        query_builder.push(column).push_bind(v);
                    }
                }
            }
        }

        query_builder.push(" GROUP BY problems.id");

        let page = condition.page.unwrap_or(1);
        let page_size = condition.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        query_builder
            .push(" LIMIT ")
            .push_bind(page_size)
            .push(" OFFSET ")
            .push_bind(offset);

        let problems = query_builder
            .build_query_as::<Problem>()
            .fetch_all(self)
            .await?;

        for p in problems.iter() {
            println!("{:?}", p);
        }

        Ok(problems)
    }

    async fn get_problem_by_id(&self, id: &str) -> Result<Problem> {
        let problem = sqlx::query_as::<_, Problem>(
            r#"
                SELECT
                    problems.id,
                    problems.contest_id,
                    problems.problem_index AS index,
                    problems.name,
                    problems.title,
                    problems.platform,
                    problems.raw_point,
                    problems.difficulty,
                    problems.is_experimental,
                    problems.url,
                    problems.solver_count,
                    problems.submissions,
                    problems.success_rate,
                    ARRAY_REMOVE (ARRAY_AGG (technical_tags.en_name), NULL) AS tags
                FROM
                    problems
                    LEFT JOIN problem_tags ON problems.id = problem_tags.problem_id
                    LEFT JOIN technical_tags ON problem_tags.technical_tag_id = technical_tags.id
                WHERE
                    problems.id = $1
                GROUP BY
                    problems.id
                "#,
        )
        .bind(id)
        .fetch_one(self)
        .await
        .with_context(|| format!("Failed to fetch problem"))?;

        println!("{:?}", problem);

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
