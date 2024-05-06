use super::problem::Problem;
use super::value_object::platform::Platform;

#[derive(Clone, Debug)]
pub struct Contest {
    // The naming convention for this field is:
    // <platform>_<contest_name>
    pub id: String,

    // The naming convention for this field is:
    // <contest_name>
    pub name: String,

    pub platform: Platform,

    pub phase: String,

    // contest start time in unix time
    pub start_time_seconds: u64,

    // contest duration in seconds
    pub duration_seconds: u64,

    pub url: String,

    pub problems: Vec<Problem>,
}

impl Contest {
    fn new(
        id: String,
        name: String,
        platform: Platform,
        phase: String,
        start_time_seconds: u64,
        duration_seconds: u64,
        url: String,
        problems: Vec<Problem>,
    ) -> Self {
        Self {
            id,
            name,
            platform,
            phase,
            start_time_seconds,
            duration_seconds,
            url,
            problems,
        }
    }

    pub fn reconstruct(
        name: String,
        platform: Platform,
        phase: String,
        start_time_seconds: u64,
        duration_seconds: u64,
        url: String,
        problems: Vec<Problem>,
    ) -> Self {
        let id = format!("{}_{}", String::from(platform), name);

        Self {
            id,
            name,
            platform,
            phase,
            start_time_seconds,
            duration_seconds,
            url,
            problems,
        }
    }
}
