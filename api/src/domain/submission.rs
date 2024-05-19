use serde::Serialize;

use super::vo::{language::Language, platform::Platform, verdict::Verdict};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Submission {
    /// A globally unique identifier for the submission.
    ///
    /// The `id` is typically formatted as `<platform>_<submission_id>`.
    /// - Atcoder: "atcoder_123456"
    /// - Codeforces: "codeforces_654321"
    /// - Yukicoder: "yukicoder_111111"
    /// - Aoj: "aoj_222222"
    /// - YOJ: "yoj_333333"
    id: String,

    /// The identifier of the user who made the submission, specific to the platform.
    user_id: String,

    /// The programming language used for the submission.
    language: String,

    /// The raw representation of the programming language used for the submission as returned by the platform.
    raw_language: String,

    /// The platform from which the submission was made.
    platform: Platform,

    /// The result of the submission.
    verdict: Verdict,

    /// The time taken to execute the submission in milliseconds.
    execution_time: Option<u64>,

    /// Optional memory usage of the submission in kilobytes.
    memory: Option<u64>,

    /// Optional size of the submitted code in bytes.
    code_size: Option<u64>, // in bytes

    /// The date and time when the submission was made in Unix time seconds.
    submission_date: u64,

    /// problem related to the submission
    problem: ProblemInfo,
}

impl Submission {
    // Submission data is fetched from the platform and reconstructed into a Submission struct.
    // Hence, there is no need to manually construct a new submission instance.
    fn _new() -> Self {
        unimplemented!()
    }

    /// Reconstructs a Submission instance from raw data fetched from a platform.
    pub fn reconstruct(
        platform: Platform,
        raw_id: u64,
        raw_user_id: &str,
        raw_language: &str,
        raw_verdict: &str,
        raw_memory: Option<u64>,
        raw_code_size: Option<u64>,
        raw_execution_time: Option<u64>,
        raw_submission_date: u64,
        raw_contest_id: Option<u64>,
        raw_problem_index: Option<String>,
        raw_problem_name: Option<String>,
        raw_point: Option<f64>,
        raw_difficulty: Option<f64>,
    ) -> Self {
        let id = String::from(platform) + "_" + &raw_id.to_string();
        let language = String::from(Language::from(raw_language));

        let (memory, execution_time, code_size, submission_date) = match platform {
            Platform::Atcoder => (None, raw_execution_time, raw_code_size, raw_submission_date),
            Platform::Codeforces => (
                Some(raw_memory.unwrap() / 1024),
                raw_execution_time,
                None,
                raw_submission_date,
            ),
            Platform::Aoj => (
                raw_memory,
                Some(raw_execution_time.unwrap() * 10),
                raw_code_size,
                raw_submission_date / 1000,
            ),
            _ => panic!("Platform not supported: {:?}", platform),
        };

        // let problem_id = problem_id.split('_').collect::<Vec<&str>>()[1].to_string();

        Self {
            id,
            user_id: raw_user_id.to_string(),
            language,
            raw_language: raw_language.to_string(),
            platform,
            verdict: Verdict::from(raw_verdict),
            memory,
            code_size,
            execution_time,
            submission_date,
        }
    }
}

/// Minimal information about a problem related to a submission.
/// Intended for use only when the submission data is fetched from the platform.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(self) struct ProblemInfo {
    id: Option<String>,
    contest_id: Option<String>,
    index: Option<String>,
    name: Option<String>,
    title: Option<String>,
    platform: Option<Platform>,
    raw_point: Option<f64>,
    difficulty: Option<f64>,
}
