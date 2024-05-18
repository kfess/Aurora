use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct YOJCategory {
    pub name: String,

    #[serde(rename = "problems")]
    pub raw_problems: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct ProblemCategories {
    pub categories: Vec<YOJCategory>,
}
