use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojProblem {
    pub id: String,

    pub available: u64,

    pub doctype: u64,

    pub name: String,

    pub problem_time_limit: u64,

    pub problem_memory_limit: u64,

    pub max_score: u64,

    pub solved_user: u64,

    pub submissions: u64,

    pub recommendations: u64,

    pub is_solved: bool,

    pub bookmark: bool,

    pub recommend: bool,

    pub success_rate: f64,

    pub score: f64,

    pub user_score: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojSubmission {
    pub judge_id: u64,

    pub judge_type: u16,

    pub user_id: String,

    pub problem_id: String,

    pub submission_date: u64, // unix time in seconds

    pub language: String,

    pub status: u16,

    pub cpu_time: u64,

    pub memory: u64,

    pub code_size: u64,

    pub accuracy: String,

    pub judge_date: u64, // unix time in seconds

    pub score: Option<u64>,

    pub problem_title: Option<String>,

    pub token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojVolume {
    pub progress: f32,

    pub number_of_problems: u16,

    pub number_of_solved: u16,

    pub problems: Vec<AojProblem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojVolumesChallengesList {
    pub volumes: Vec<u16>,

    pub large_cls: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojChallenges {
    pub large_cls: Vec<AojLargeCl>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojLargeCl {
    pub id: String,

    pub title: String,

    pub filter: Option<Vec<AojFilter>>,

    pub middle_cls: Option<Vec<AojMiddleCls>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojMiddleCls {
    pub id: String,

    pub number_of_problems: u16,

    pub number_of_solved: u16,

    pub progress: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AojFilter {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojChallengesAndRelatedContests {
    pub large_cl: AojLargeCl,

    pub middle_cls: Option<Vec<AojMiddleCls>>,

    pub contests: Vec<AojContest>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojContest {
    pub abbr: String,

    pub large_cl: String,

    pub middle_cl: String,

    pub year: u16,

    pub progress: f32,

    pub number_of_problems: u16,

    pub number_of_solved: u16,

    pub days: Vec<AojDay>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AojDay {
    pub id: u64,

    pub day: u16,

    pub title: String,

    pub progress: f32,

    pub number_of_problems: u16,

    pub number_of_solved: u16,

    pub problems: Vec<AojProblem>,
}
