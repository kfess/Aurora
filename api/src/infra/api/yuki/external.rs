use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YukicoderProblem {
    pub no: u64,

    pub problem_id: u64,

    pub title: String,

    pub author_id: u64,

    pub tester_ids: String,

    pub level: f64,

    pub problem_type: u64,

    pub tags: String,

    pub date: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YukicoderProblemWithStatistics {
    pub no: u64,

    pub problem_id: u64,

    pub title: String,

    pub author_id: u64,

    pub tester_ids: String,

    pub level: f64,

    pub problem_type: u64,

    pub tags: String,

    pub date: String,

    pub statistics: YukicoderStatistics,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YukicoderContest {
    pub id: u64,

    pub name: String,

    pub date: String,

    pub end_date: String,

    pub problem_id_list: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YukicoderTag {
    pub key: String,

    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YukicoderStatistics {
    pub total: i32,

    pub solved: i32,

    pub first_accepted_time_second: i64,

    #[serde(rename = "FirstACSubmissionId")]
    pub first_ac_submission_id: u64,

    pub short_code_submission_id: u64,

    pub pure_short_code_submission_id: u64,

    pub fast_submission_id: u64,
}
