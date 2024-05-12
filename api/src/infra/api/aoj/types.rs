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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AojSubmission {
    #[serde(rename = "judgeId")]
    pub judge_id: u64,

    #[serde(rename = "judgeType")]
    pub judge_type: u16,

    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "problemId")]
    pub problem_id: String,

    #[serde(rename = "submissionDate")]
    pub submission_date: u64, // unix time in seconds

    #[serde(rename = "language")]
    pub language: String,

    #[serde(rename = "status")]
    pub status: u16,

    #[serde(rename = "cpuTime")]
    pub cpu_time: u64,

    #[serde(rename = "memory")]
    pub memory: u64,

    #[serde(rename = "codeSize")]
    pub code_size: u64,

    #[serde(rename = "accuracy")]
    pub accuracy: String,

    #[serde(rename = "judgeDate")]
    pub judge_date: u64, // unix time in seconds

    #[serde(rename = "score")]
    pub score: u64,

    #[serde(rename = "problemTitle")]
    pub problem_title: Option<String>,

    #[serde(rename = "token")]
    pub token: Option<String>,
}
