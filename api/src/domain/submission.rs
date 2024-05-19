use super::vo::{language::Language, platform::Platform, verdict::Verdict};

// We do not include contest_id in the submission struct
#[derive(Clone, Debug, PartialEq)]
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

    /// The identifier of the problem associated with this submission.
    problem_id: String,
}

impl Submission {
    pub fn new(
        id: String,
        user_id: String,
        language: String,
        raw_language: String,
        platform: Platform,
        verdict: Verdict,
        memory: Option<u64>,
        code_size: Option<u64>,
        execution_time: Option<u64>,
        submission_date: u64,
        problem_id: String,
    ) -> Self {
        Self {
            id,
            user_id,
            language,
            raw_language,
            platform,
            verdict,
            memory,
            code_size,
            execution_time,
            submission_date,
            problem_id,
        }
    }

    pub fn reconstruct(
        raw_id: u64,
        user_id: &str,
        raw_language: &str,
        platform: Platform,
        verdict: Verdict,
        raw_memory: Option<u64>,
        raw_code_size: Option<u64>,
        raw_execution_time: Option<u64>,
        raw_submission_date: u64,
        problem_id: &str,
    ) -> Self {
        let id = String::from(platform) + "_" + &raw_id.to_string();
        let language = String::from(Language::from(raw_language));

        let (memory, execution_time, code_size, submission_date) = match platform {
            Platform::Atcoder => (None, raw_execution_time, None, raw_submission_date),
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

        let problem_id = problem_id.split('_').collect::<Vec<&str>>()[1].to_string();

        Self {
            id,
            user_id: user_id.to_string(),
            language,
            raw_language: raw_language.to_string(),
            platform,
            verdict,
            memory,
            code_size,
            execution_time,
            submission_date,
            problem_id,
        }
    }
}
