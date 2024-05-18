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
