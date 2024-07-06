#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthProvider {
    Google,
    Github,
}

impl TryFrom<&str> for AuthProvider {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "google" => Ok(AuthProvider::Google),
            "github" => Ok(AuthProvider::Github),
            _ => Err(format!("Unknown provider: {}", value)),
        }
    }
}

impl TryFrom<&String> for AuthProvider {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "google" => Ok(AuthProvider::Google),
            "github" => Ok(AuthProvider::Github),
            _ => Err(format!("Unknown provider: {}", value)),
        }
    }
}
