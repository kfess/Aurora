// convert external API data to domain data

use crate::domain::{problem::Problem, value_object::platform::Platform};

use super::types::AojProblem;

impl std::convert::TryFrom<AojProblem> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: AojProblem) -> Result<Self, Self::Error> {
        // Ok(Problem::reconstruct(value.id, Platform::Aoj, value.name))
        Ok(Problem::reconstruct(
            value.id,
            "contest_id".to_string(),
            "index".to_string(),
            "name".to_string(),
            "title".to_string(),
            Platform::Aoj,
            Some(0.0),
            Some(0.0),
            vec![],
            "".to_string(),
            Some(0),
            Some(0),
            Some(0.0),
        ))
    }
}
