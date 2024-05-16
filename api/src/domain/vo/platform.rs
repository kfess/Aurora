#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Platform {
    Atcoder,
    Codeforces,
    Yukicoder,
    Aoj,
    YOJ,
}

impl std::convert::From<&str> for Platform {
    fn from(value: &str) -> Self {
        match value {
            "atcoder" => Platform::Atcoder,
            "codeforces" => Platform::Codeforces,
            "yukicoder" => Platform::Yukicoder,
            "aoj" => Platform::Aoj,
            "yosupo_online_judge" => Platform::YOJ,
            _ => panic!("Invalid platform: {}", value),
        }
    }
}

impl std::convert::From<Platform> for String {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Atcoder => "atcoder".to_string(),
            Platform::Codeforces => "codeforces".to_string(),
            Platform::Yukicoder => "yukicoder".to_string(),
            Platform::Aoj => "aoj".to_string(),
            Platform::YOJ => "yosupo_online_judge".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for (v, s) in [
            (Platform::Atcoder, "atcoder"),
            (Platform::Codeforces, "codeforces"),
            (Platform::Yukicoder, "yukicoder"),
            (Platform::Aoj, "aoj"),
            (Platform::YosupoOnlineJudge, "yosupo_online_judge"),
        ] {
            assert_eq!(Platform::from(s), v);
            assert_eq!(String::from(v), s);
        }
    }
}
