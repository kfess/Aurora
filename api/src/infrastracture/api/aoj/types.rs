use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AojProblem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "available")]
    pub available: u64,

    #[serde(rename = "doctype")]
    pub doctype: u64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "problemTimeLimit")]
    pub problem_time_limit: u64,

    #[serde(rename = "problemMemoryLimit")]
    pub problem_memory_limit: u64,

    #[serde(rename = "maxScore")]
    pub max_score: u64,

    #[serde(rename = "solvedUser")]
    pub solved_user: u64,

    #[serde(rename = "submissions")]
    pub submissions: u64,

    #[serde(rename = "recommendations")]
    pub recommendations: u64,

    #[serde(rename = "isSolved")]
    pub is_solved: bool,

    #[serde(rename = "bookmark")]
    pub bookmark: bool,

    #[serde(rename = "recommend")]
    pub recommend: bool,

    #[serde(rename = "successRate")]
    pub success_rate: f64,

    #[serde(rename = "score")]
    pub score: f64,

    #[serde(rename = "userScore")]
    pub user_score: f64,
}
