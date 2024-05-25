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

    /// The identifier of the submission, specific to the platform.
    /// - Atcoder: 123456
    /// - Codeforces: 654321
    /// - Yukicoder: 111111
    /// - Aoj: 222222
    /// - YOJ: 333333
    raw_id: String,

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
        id: String,
        user_id: String,
        language: String,
        verdict: Verdict,
        execution_time: Option<u64>,
        memory: Option<u64>,
        code_size: Option<u64>,
        submission_date: u64,
        contest_id: Option<String>,
        index: Option<String>,
        name: Option<String>,
        raw_point: Option<f64>,
        difficulty: Option<f64>,
    ) -> Self {
        Self {
            id: String::from(platform) + "_" + &id.to_string(),
            raw_id: id,
            user_id,
            language: String::from(Language::from(language.as_str())),
            raw_language: language,
            platform,
            verdict,
            execution_time,
            memory,
            code_size,
            submission_date,
            problem: ProblemInfo {
                contest_id,
                index,
                name,
                raw_point,
                difficulty,
            },
        }
    }
}

/// Minimal information about a problem related to a submission.
/// Intended for use only when the submission data is fetched from the platform.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(self) struct ProblemInfo {
    contest_id: Option<String>,
    index: Option<String>,
    name: Option<String>,
    raw_point: Option<f64>,
    difficulty: Option<f64>,
}
