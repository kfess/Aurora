use std::collections::HashMap;

use anyhow::{Context, Result};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};

use crate::domain::{contest::Contest, problem::Problem};

pub struct Condition<'a> {
    pub platform: Option<&'a str>,
    pub category: Option<&'a str>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl Default for Condition<'_> {
    fn default() -> Self {
        Self {
            platform: Some("atcoder"),
            category: None,
            page: None,
            per_page: None,
        }
    }
}

enum BindValue<'a> {
    Str(&'a str),
    Int(i32),
}

#[trait_variant::make]
pub trait ContestRepository {
    async fn get_contests_by_condition(&self, condition: &Condition<'_>) -> Result<Vec<Contest>>;
    async fn update_contests(&self, contests: &Vec<Contest>) -> Result<()>;
}

impl ContestRepository for PgPool {
    async fn get_contests_by_condition(&self, condition: &Condition<'_>) -> Result<Vec<Contest>> {
        // タグ情報は必要ないので、空の配列を返す
        let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
            r#"
            SELECT
                contests.id,
                contests.raw_id,
                contests.name,
                contests.category,
                contests.platform,
                contests.phase,
                contests.start_time_seconds,
                contests.duration_seconds,
                contests.url,
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
                ARRAY[]::VARCHAR[] AS tags
            FROM
                contests
                LEFT JOIN contest_problems ON contests.id = contest_problems.contest_id
                LEFT JOIN problems ON contest_problems.problem_id = problems.id
            "#,
        );

        let mut conditions: Vec<(&str, BindValue)> = Vec::new();

        if let Some(platform) = condition.platform {
            conditions.push(("contests.platform = ", BindValue::Str(platform)));
        }

        if let Some(category) = condition.category {
            conditions.push(("contests.category = ", BindValue::Str(category)));
        }

        if !conditions.is_empty() {
            query_builder.push(" WHERE ");

            for (i, (column, value)) in conditions.iter().enumerate() {
                if i != 0 {
                    query_builder.push(" AND ");
                }

                match value {
                    BindValue::Str(value) => {
                        query_builder.push(column).push_bind(value);
                    }
                    BindValue::Int(value) => {
                        query_builder.push(column).push_bind(value);
                    }
                }
            }
        }

        let page = condition.page.unwrap_or(1);
        let per_page = condition.per_page.unwrap_or(100);
        let offset = (page - 1) * per_page;

        query_builder
            .push(" LIMIT ")
            .push_bind(per_page)
            .push(" OFFSET ")
            .push_bind(offset);

        let query = query_builder.build();

        println!("{:?}", query.sql());

        let rows = sqlx::query(&query.sql()).fetch_all(self).await?;

        let mut contests_map: HashMap<String, Contest> = HashMap::new();

        for row in rows {
            let contest_id = row.get("contests.id");
            let contest = contests_map.entry(contest_id).or_insert_with(|| {
                Contest::reconstruct_from_db_wo_problems(
                    row.get("contests.id"),
                    row.get("contests.raw_id"),
                    row.get("contests.name"),
                    row.get("contests.category"),
                    row.get("contests.platform"),
                    row.get("contests.phase"),
                    row.get("contests.start_time_seconds"),
                    row.get("contests.duration_seconds"),
                    row.get("contests.url"),
                )
            });

            if let Some(problem_id) = row.get::<Option<String>, _>("problems.id") {
                let problem = Problem::reconstruct_from_db(
                    problem_id,
                    row.get("problems.contest_id"),
                    row.get("problems.index"),
                    row.get("problems.name"),
                    row.get("problems.title"),
                    row.get("problems.platform"),
                    row.get("problems.raw_point"),
                    row.get("problems.difficulty"),
                    row.get("problems.is_experimental"),
                    row.get("problems.tags"),
                    row.get("problems.url"),
                    row.get("problems.solver_count"),
                    row.get("problems.submissions"),
                    row.get("problems.success_rate"),
                );

                contest.problems.push(problem);
            }
        }

        let contests: Vec<Contest> = contests_map.into_iter().map(|(_, v)| v).collect();

        for c in contests.iter() {
            println!("{:?}", c);
        }

        Ok(contests)
    }

    async fn update_contests(&self, contests: &Vec<Contest>) -> Result<()> {
        let mut transaction = self.begin().await.unwrap();

        for chunk in contests.chunks(100) {
            // contests テーブルの更新
            {
                let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
                    r#"
                INSERT INTO contests (
                    id, raw_id, name, category, platform, phase, start_time_seconds, duration_seconds, url
                )
                "#,
                );

                query_builder.push_values(chunk, |mut separated, contest| {
                    separated.push_bind(&contest.id);
                    separated.push_bind(&contest.raw_id);
                    separated.push_bind(&contest.name);
                    separated.push_bind(&contest.category);
                    separated.push_bind(&contest.platform);
                    separated.push_bind(&contest.phase);
                    separated.push_bind(&contest.start_time_seconds);
                    separated.push_bind(&contest.duration_seconds);
                    separated.push_bind(&contest.url);
                });

                query_builder.push(
                    r#"
                ON CONFLICT (id) DO UPDATE SET
                    raw_id = EXCLUDED.raw_id,
                    name = EXCLUDED.name,
                    category = EXCLUDED.category,
                    platform = EXCLUDED.platform,
                    phase = EXCLUDED.phase,
                    start_time_seconds = EXCLUDED.start_time_seconds,
                    duration_seconds = EXCLUDED.duration_seconds,
                    url = EXCLUDED.url
                "#,
                );

                let query = query_builder.build();
                query
                    .execute(&mut *transaction)
                    .await
                    .with_context(|| format!("Failed to update contests: {:?}", chunk))?;
            }

            // contest_problems テーブルの更新
            {
                let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new(
                    r#"
                INSERT INTO contest_problems (contest_id, problem_id)
                "#,
                );

                let problems = chunk
                    .iter()
                    .flat_map(|contest| contest.problems.iter())
                    .collect::<Vec<&Problem>>();

                query_builder.push_values(problems, |mut separated, problem| {
                    separated
                        .push_bind(&problem.contest_id)
                        .push_bind(&problem.id);
                });

                query_builder.push(
                    r#"
                ON CONFLICT (contest_id, problem_id) DO UPDATE SET
                    contest_id = EXCLUDED.contest_id,
                    problem_id = EXCLUDED.problem_id
                "#,
                );

                let query = query_builder.build();
                query
                    .execute(&mut *transaction)
                    .await
                    .with_context(|| format!("Failed to update contest_problems: {:?}", chunk))?;
            }
        }

        transaction.commit().await.unwrap();

        Ok(())
    }
}
