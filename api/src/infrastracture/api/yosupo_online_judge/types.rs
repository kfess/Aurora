use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitHubRepoContent {
    pub name: String,
    path: String,
    sha: String,
    size: u64,
    url: String,
    html_url: String,
    git_url: String,
    download_url: Option<String>,

    #[serde(rename = "type")]
    _type: String,
    _links: GitHubRepoContentLinks,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitHubRepoContentLinks {
    #[serde(rename = "self")]
    self_link: String,
    git: String,
    html: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct YosupoOnlineJudgeProblem {
    pub name: String,
    pub category: String,
}
