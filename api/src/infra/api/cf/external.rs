use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

// Codeforces API Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    OK,
    FAILED,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesAPIResponse<T> {
    pub status: Status,
    pub result: Option<T>,
}

// Codeforces Problem Tags

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Tag {
    Implementation,
    Math,
    Greedy,
    Dp,
    DataStructures,
    BruteForce,
    ConstructiveAlgorithms,
    Graph,
    Sortings,
    BinarySearch,
    DfsAndSimilar,
    Trees,
    Strings,
    NumberTheory,
    Combinatorics,
    Special,
    Geometry,
    Bitmasks,
    TwoPointers,
    Dsu,
    ShortestPaths,
    Probabilities,
    DivideAndConquer,
    Hashing,
    Games,
    Flows,
    Interactive,
    Martices,
    StringSuffixStructures,
    Fft,
    GraphMatchings,
    TernarySearch,
    ExpressionParsing,
    MeetInTheMiddle,
    TwoSat,
    ChineseRemainderTheorem,
    Schedules,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProblemType {
    PROGRAMMING,
    QUESTION,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblem {
    #[serde(rename = "contestId")]
    pub contest_id: u64,

    #[serde(rename = "problemsetName")]
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
pub struct CodeforcesProblemStat {
    #[serde(rename = "contestId")]
    pub contest_id: u64,

    pub index: String,

    #[serde(rename = "solvedCount")]
    pub solved_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblemResult {
    pub problems: Vec<CodeforcesProblem>,

    #[serde(rename = "problemStatistics")]
    pub problem_statistics: Vec<CodeforcesProblemStat>,
}

pub type CodeforcesProblemResponse = CodeforcesAPIResponse<CodeforcesProblemResult>;

// Codeforces Contest

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesContest {
    pub id: u64,

    pub name: String,

    #[serde(rename = "type")]
    pub r#type: String,

    pub phase: String,

    pub frozen: bool,

    #[serde(rename = "durationSeconds")]
    pub duration_seconds: u64,

    #[serde(rename = "startTimeSeconds")]
    pub start_time_seconds: Option<i64>,

    #[serde(rename = "relativeTimeSeconds")]
    pub relative_time_seconds: Option<i64>,

    #[serde(rename = "preparedBy")]
    pub prepared_by: Option<String>,

    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,

    pub description: Option<String>,

    pub difficulty: Option<u64>,

    pub kind: Option<String>,

    #[serde(rename = "icpcRegion")]
    pub icpc_region: Option<String>,

    pub country: Option<String>,

    pub city: Option<String>,

    pub season: Option<String>,
}

pub type CodeforcesContestResponse = CodeforcesAPIResponse<Vec<CodeforcesContest>>;
