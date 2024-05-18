use std::convert::From;

/// `Phase` is a value object that represents the phase of the project.
pub enum Phase {
    Before,
    Finished,
    Coding,
    Unknown,
}

impl From<&str> for Phase {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "before" => Phase::Before,
            "finished" => Phase::Finished,
            "coding" => Phase::Coding,
            _ => Phase::Unknown,
        }
    }
}

impl From<Phase> for String {
    fn from(value: Phase) -> Self {
        match value {
            Phase::Before => "before".to_string(),
            Phase::Finished => "finished".to_string(),
            Phase::Coding => "coding".to_string(),
            Phase::Unknown => "unknown".to_string(),
        }
    }
}
