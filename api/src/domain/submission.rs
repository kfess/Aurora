use super::vo::{language::Language, platform::Platform, verdict::Verdict};

// We do not include contest_id in the submission struct
#[derive(Clone, Debug, PartialEq)]
pub struct Submission {
    // The naming convention for this field is:
    // <platform>_<submission_id>
    id: String,

    user_id: String, // user_id specific to the platform

    language: String,

    raw_language: String,

    platform: Platform,

    verdict: Verdict,

    execution_time: u64, // in ms

    memory: Option<u64>, // in KBytes

    code_size: Option<u64>, // in bytes

    submission_date: u64, // unix time in seconds

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
        execution_time: u64,
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
        raw_execution_time: u64,
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
                raw_execution_time * 10,
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
