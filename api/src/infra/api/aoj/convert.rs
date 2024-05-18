// convert external API data to domain data

use crate::domain::{problem::Problem, vo::platform::Platform};

use super::types::AojProblem;

impl std::convert::TryFrom<AojProblem> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: AojProblem) -> Result<Self, Self::Error> {
        Ok(Problem::reconstruct(
            "contest_id".to_string(),
            "index".to_string(),
            "name".to_string(),
            Platform::Aoj,
            Some(0.0),
            Some(0.0),
            Some(false),
            vec![],
            "".to_string(),
            Some(0),
            Some(0),
        ))
    }
}
