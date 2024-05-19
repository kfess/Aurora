use regex::Regex;
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    Testing,
    CompileError,
    TimeLimitExceeded,
    RuntimeError,
    MemoryLimitExceeded,
    Waiting,
    OutputLimit,
    WaitingRejudge,
    QueryLimitExceeded,
    PresentationError,
    Failed,
    Partial,
    IdlenessLimitExceeded,
    SecurityViolated,
    Crashed,
    InputPresentationCrashed,
    Challenged,
    Skipped,
    Rejected,
    ImplementationError,
    Unknown,
}

impl From<&str> for Verdict {
    fn from(value: &str) -> Self {
        match value {
            "AC" | "OK" => Verdict::Accepted,
            "WA" => Verdict::WrongAnswer,
            value if value == "TESTING" || Regex::new(r"^\d").unwrap().is_match(value) => {
                Verdict::Testing
            }
            "CE" => Verdict::CompileError,
            "TLE" => Verdict::TimeLimitExceeded,
            "RE" => Verdict::RuntimeError,
            "MLE" => Verdict::MemoryLimitExceeded,
            "WJ" => Verdict::Waiting,
            "OLE" => Verdict::OutputLimit,
            "WR" => Verdict::WaitingRejudge,
            "QLE" => Verdict::QueryLimitExceeded,
            "PE" => Verdict::PresentationError,
            "FAILED" => Verdict::Failed,
            "PARTIAL" => Verdict::Partial,
            "ILE" => Verdict::IdlenessLimitExceeded,
            "SV" => Verdict::SecurityViolated,
            "CRASHED" => Verdict::Crashed,
            "IPC" => Verdict::InputPresentationCrashed,
            "CHALLENGED" => Verdict::Challenged,
            "SKIPPED" => Verdict::Skipped,
            "REJECTED" => Verdict::Rejected,
            "IE" => Verdict::ImplementationError,
            _ => Verdict::Unknown,
        }
    }
}

impl From<Verdict> for String {
    fn from(value: Verdict) -> Self {
        match value {
            Verdict::Accepted => "AC".to_string(),
            Verdict::WrongAnswer => "WA".to_string(),
            Verdict::Testing => "TESTING".to_string(),
            Verdict::CompileError => "CE".to_string(),
            Verdict::TimeLimitExceeded => "TLE".to_string(),
            Verdict::RuntimeError => "RE".to_string(),
            Verdict::MemoryLimitExceeded => "MLE".to_string(),
            Verdict::Waiting => "WJ".to_string(),
            Verdict::OutputLimit => "OLE".to_string(),
            Verdict::WaitingRejudge => "WR".to_string(),
            Verdict::QueryLimitExceeded => "QLE".to_string(),
            Verdict::PresentationError => "PE".to_string(),
            Verdict::Failed => "FAILED".to_string(),
            Verdict::Partial => "PARTIAL".to_string(),
            Verdict::IdlenessLimitExceeded => "ILE".to_string(),
            Verdict::SecurityViolated => "SV".to_string(),
            Verdict::Crashed => "CRASHED".to_string(),
            Verdict::InputPresentationCrashed => "IPC".to_string(),
            Verdict::Challenged => "CHALLENGED".to_string(),
            Verdict::Skipped => "SKIPPED".to_string(),
            Verdict::Rejected => "REJECTED".to_string(),
            Verdict::ImplementationError => "IE".to_string(),
            Verdict::Unknown => "UNKNOWN".to_string(),
        }
    }
}
