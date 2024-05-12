#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    CompileError,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    Accepted,
    Waiting,
    OutputLimit,
    RuntimeError,
    PresentationError,
    Failed,
    Ok,
    Partial,
    IdlenessLimitExceeded,
    SecurityViolated,
    Crashed,
    InputPresentationCrashed,
    Challenged,
    Skipped,
    Testing,
    Rejected,
    Unknown,
}

impl std::convert::From<&str> for Verdict {
    fn from(value: &str) -> Self {
        match value {
            "CE" => Verdict::CompileError,
            "WA" => Verdict::WrongAnswer,
            "TLE" => Verdict::TimeLimitExceeded,
            "MLE" => Verdict::MemoryLimitExceeded,
            "AC" => Verdict::Accepted,
            "WJ" => Verdict::Waiting,
            "OLE" => Verdict::OutputLimit,
            "RE" => Verdict::RuntimeError,
            "PE" => Verdict::PresentationError,
            "FAILED" => Verdict::Failed,
            "OK" => Verdict::Ok,
            "PARTIAL" => Verdict::Partial,
            "ILE" => Verdict::IdlenessLimitExceeded,
            "SV" => Verdict::SecurityViolated,
            "CRASHED" => Verdict::Crashed,
            "IPC" => Verdict::InputPresentationCrashed,
            "CHALLENGED" => Verdict::Challenged,
            "SKIPPED" => Verdict::Skipped,
            "TESTING" => Verdict::Testing,
            "REJECTED" => Verdict::Rejected,
            _ => Verdict::Unknown,
        }
    }
}

impl std::convert::From<Verdict> for String {
    fn from(value: Verdict) -> Self {
        match value {
            Verdict::CompileError => "CE".to_string(),
            Verdict::WrongAnswer => "WA".to_string(),
            Verdict::TimeLimitExceeded => "TLE".to_string(),
            Verdict::MemoryLimitExceeded => "MLE".to_string(),
            Verdict::Accepted => "AC".to_string(),
            Verdict::Waiting => "WJ".to_string(),
            Verdict::OutputLimit => "OLE".to_string(),
            Verdict::RuntimeError => "RE".to_string(),
            Verdict::PresentationError => "PE".to_string(),
            Verdict::Failed => "FAILED".to_string(),
            Verdict::Ok => "OK".to_string(),
            Verdict::Partial => "PARTIAL".to_string(),
            Verdict::IdlenessLimitExceeded => "ILE".to_string(),
            Verdict::SecurityViolated => "SV".to_string(),
            Verdict::Crashed => "CRASHED".to_string(),
            Verdict::InputPresentationCrashed => "IPC".to_string(),
            Verdict::Challenged => "CHALLENGED".to_string(),
            Verdict::Skipped => "SKIPPED".to_string(),
            Verdict::Testing => "TESTING".to_string(),
            Verdict::Rejected => "REJECTED".to_string(),
            Verdict::Unknown => "UNKNOWN".to_string(),
        }
    }
}
