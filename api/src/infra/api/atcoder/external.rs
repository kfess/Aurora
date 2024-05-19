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

    pub shortest_submission_id: String,

    pub shortest_contest_id: String,

    pub shortest_user_id: String,

    pub fastest_submission_id: String,

    pub fastest_contest_id: String,

    pub fastest_user_id: String,

    pub first_submission_id: String,

    pub first_contest_id: String,

    pub first_user_id: String,

    pub source_code_length: u64,

    pub execution_time: u64,

    pub point: f64,

    pub solver_count: u64,
}

/// `AtcoderContest` is a struct that contains more detailed information about a contest.
///
/// For the raw data, see:
/// [AtCoder Problems JSON](https://kenkoooo.com/atcoder/resources/contests.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtcoderContest {
    pub id: String,

    pub start_epoch_second: u64,

    pub duration_second: u64,

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
    #[serde(skip_deserializing)]
    pub slope: f64,

    #[serde(skip_deserializing)]
    pub intercept: f64,

    #[serde(skip_deserializing)]
    pub variance: f64,

    pub difficulty: f64,

    #[serde(skip_deserializing)]
    pub discrimination: f64,

    #[serde(skip_deserializing)]
    pub irt_log_likelihood: f64,

    #[serde(skip_deserializing)]
    pub irt_users: u64,

    pub is_experimental: bool,
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

    #[serde(skip_deserializing)]
    pub point: f64,

    pub length: u64, // bytes

    pub result: String,

    pub execution_time: Option<u64>, // ms
}
