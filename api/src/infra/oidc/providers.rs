#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthProvider {
    Google,
    Facebook,
    Github,
}

impl TryFrom<&str> for AuthProvider {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "google" => Ok(AuthProvider::Google),
            "facebook" => Ok(AuthProvider::Facebook),
            "github" => Ok(AuthProvider::Github),
            _ => Err(format!("Unknown provider: {}", value)),
        }
    }
}
