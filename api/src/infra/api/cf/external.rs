use serde::{Deserialize, Serialize};

// Codeforces API Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) enum Status {
    OK,
    FAILED,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesAPIResponse<T> {
    pub status: Status,
    pub result: Option<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) enum ProblemType {
    PROGRAMMING,
    QUESTION,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesProblem {
    pub contest_id: u64,

    pub problemset_name: Option<String>,

    pub index: String,

    pub name: String,

    #[serde(rename = "type")]
    pub problem_type: ProblemType,

    pub points: Option<f64>,

    pub rating: Option<f64>,

    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesProblemStat {
    pub contest_id: u64,

    pub index: String,

    pub solved_count: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesProblemResult {
    pub problems: Vec<CodeforcesProblem>,

    pub problem_statistics: Vec<CodeforcesProblemStat>,
}

pub(super) type CodeforcesProblemResponse = CodeforcesAPIResponse<CodeforcesProblemResult>;

// Codeforces Contest

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesContest {
    pub id: u64,

    pub name: String,

    pub r#type: String,

    pub phase: String,

    pub frozen: bool,

    pub duration_seconds: u64,

    pub start_time_seconds: Option<i64>,

    pub relative_time_seconds: Option<i64>,

    pub prepared_by: Option<String>,

    pub website_url: Option<String>,

    pub description: Option<String>,

    pub difficulty: Option<u64>,

    pub kind: Option<String>,

    pub icpc_region: Option<String>,

    pub country: Option<String>,

    pub city: Option<String>,

    pub season: Option<String>,
}

pub(super) type CodeforcesContestResponse = CodeforcesAPIResponse<Vec<CodeforcesContest>>;

// Codeforces Submission
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesSubmission {
    pub id: u64,

    pub contest_id: Option<u64>,

    pub creation_time_seconds: u64,

    pub relative_time_seconds: u64,

    pub problem: CodeforcesProblem,

    pub author: CodeforcesParty,

    pub programming_language: String,

    pub verdict: String,

    pub testset: String,

    pub passed_test_count: u64,

    pub time_consumed_millis: u64,

    pub memory_consumed_bytes: u64,

    pub points: Option<f64>,
}

pub(super) type CodeforcesSubmissionResponse = CodeforcesAPIResponse<Vec<CodeforcesSubmission>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesParty {
    pub contest_id: Option<u64>,

    pub members: Vec<CodeforcesMember>,

    pub participant_type: String,

    pub team_id: Option<u64>,

    pub team_name: Option<String>,

    pub ghost: bool,

    pub room: Option<u64>,

    pub start_time_seconds: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CodeforcesMember {
    pub handle: String,

    pub name: Option<String>,
}
