use anyhow::Result;
use reqwest::header::{self, HeaderMap};

pub fn build_github_header() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, "application/vnd.github+json".parse()?);
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    headers.insert(header::USER_AGENT, "reqwest".parse()?);

    Ok(headers)
}
