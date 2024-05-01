use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YukicoderProblem {
    #[serde(rename = "No")]
    pub no: u64,

    #[serde(rename = "ProblemId")]
    pub problem_id: u64,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "AuthorId")]
    pub author_id: u64,

    #[serde(rename = "TesterIds")]
    pub tester_ids: String,

    #[serde(rename = "Level")]
    pub level: f64,

    #[serde(rename = "ProblemType")]
    pub problem_type: u64,

    #[serde(rename = "Tags")]
    pub tags: String,

    #[serde(rename = "Date")]
    pub date: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YukicoderProblemWithStatistics {
    #[serde(rename = "No")]
    pub no: u64,

    #[serde(rename = "ProblemId")]
    pub problem_id: u64,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "AuthorId")]
    pub author_id: u64,

    #[serde(rename = "TesterIds")]
    pub tester_ids: String,

    #[serde(rename = "Level")]
    pub level: f64,

    #[serde(rename = "ProblemType")]
    pub problem_type: u64,

    #[serde(rename = "Tags")]
    pub tags: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Statistics")]
    pub statistics: YukicoderStatistics,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YukicoderContest {
    #[serde(rename = "Id")]
    pub id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "EndDate")]
    pub end_date: String,

    #[serde(rename = "ProblemIdList")]
    pub problem_id_list: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct YukicoderTag {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "Count")]
    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YukicoderStatistics {
    #[serde(rename = "Total")]
    pub total: u64,

    #[serde(rename = "Solved")]
    pub solved: u64,

    #[serde(rename = "FirstAcceptedTimeSecond")]
    pub first_accepted_time_second: u64,

    #[serde(rename = "FirstACSubmissionId")]
    pub first_ac_submission_id: u64,

    #[serde(rename = "ShortCodeSubmissionId")]
    pub short_code_submission_id: u64,

    #[serde(rename = "PureShortCodeSubmissionId")]
    pub pure_short_code_submission_id: u64,

    #[serde(rename = "FastSubmissionId")]
    pub fast_submission_id: u64,
}
