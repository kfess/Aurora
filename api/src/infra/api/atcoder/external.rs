use serde::{Deserialize, Serialize};

/// `AtcoderProblem` is a struct that contains more detailed information about a problem.
///
/// For the raw data, see:
/// [AtCoder Problems JSON](https://kenkoooo.com/atcoder/resources/merged-problems.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtcoderProblem {
    pub id: String,

    pub contest_id: String,

    pub problem_index: String,

    pub name: String,

    pub title: String,

    pub shortest_submission_id: Option<u32>,

    pub shortest_contest_id: Option<String>,

    pub shortest_user_id: Option<String>,

    pub fastest_submission_id: Option<u32>,

    pub fastest_contest_id: Option<String>,

    pub fastest_user_id: Option<String>,

    pub first_submission_id: Option<u32>,

    pub first_contest_id: Option<String>,

    pub first_user_id: Option<String>,

    pub source_code_length: Option<u64>,

    pub execution_time: Option<u64>,

    pub point: Option<f64>,

    pub solver_count: Option<i32>,
}

/// `AtcoderContest` is a struct that contains more detailed information about a contest.
///
/// For the raw data, see:
/// [AtCoder Problems JSON](https://kenkoooo.com/atcoder/resources/contests.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtcoderContest {
    pub id: String,

    pub start_epoch_second: i32,

    pub duration_second: i32,

    pub title: String,

    pub rate_change: String,
}

impl Default for AtcoderContest {
    fn default() -> Self {
        AtcoderContest {
            id: "default_id".to_string(),
            title: "default_title".to_string(),
            start_epoch_second: 1_468_670_401,
            duration_second: 0,
            rate_change: "-".to_string(),
        }
    }
}

/// `Estimation` is a struct that contains the estimated values of the IRT model.
///
/// For the raw data, see:
/// [AtCoder Problems JSON](https://kenkoooo.com/atcoder/resources/merged-problems.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estimation {
    pub slope: Option<f64>,

    pub intercept: Option<f64>,

    pub variance: Option<f64>,

    pub difficulty: Option<f64>,

    pub discrimination: Option<f64>,

    pub irt_loglikelihood: Option<f64>,

    pub irt_users: Option<u64>,

    pub is_experimental: Option<bool>,
}

/// `AtcoderSubmission` is a struct that contains more detailed information about a submission.
///
/// For the raw data, see:
/// [AtCoder Problems JSON](https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=chokudai&from_second=1560046356)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtcoderSubmission {
    pub id: u64,

    pub epoch_second: u64,

    pub problem_id: String,

    pub contest_id: String,

    pub user_id: String,

    pub language: String,

    pub point: f64,

    pub length: u64, // bytes

    pub result: String,

    pub execution_time: Option<u64>, // ms
}
