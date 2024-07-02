#[derive(sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: Option<String>,

    // External account IDs
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub google_id: Option<String>,
    pub google_email: Option<String>,

    // Competitive programming usernames
    pub atcoder_username: Option<String>,
    pub codeforces_username: Option<String>,
    pub yukicoder_username: Option<String>,
    pub aoj_username: Option<String>,
    pub yoj_username: Option<String>,
}
