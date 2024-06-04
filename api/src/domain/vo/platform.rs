use std::convert::From;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Decode,
    sqlx::Encode,
)]
#[sqlx(rename_all = "lowercase")]
pub enum Platform {
    Atcoder,
    Codeforces,
    Yukicoder,
    Aoj,
    YOJ,
}

impl From<&str> for Platform {
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

impl From<Platform> for String {
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

impl sqlx::Type<sqlx::Postgres> for Platform {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <&str as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <&str as sqlx::Type<sqlx::Postgres>>::compatible(ty)
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
            (Platform::YOJ, "yosupo_online_judge"),
        ] {
            assert_eq!(Platform::from(s), v);
            assert_eq!(String::from(v), s);
        }
    }
}
