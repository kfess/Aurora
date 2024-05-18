use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct YOJCategory {
    pub name: String,

    #[serde(rename = "problems")]
    pub raw_problems: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProblemCategories {
    pub categories: Vec<YOJCategory>,
}
