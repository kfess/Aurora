use super::problem::Problem;
use super::vo::platform::Platform;

#[derive(Clone, Debug)]
pub struct Contest {
    /// The naming convention for this field is <platform>_<raw_id>
    /// Atcoder: "atcoder_abc001", "atcoder_arc001", "atcoder_agc001", ...
    /// Codeforces: "codeforces_1", "codeforces_2", "codeforces_3", "codeforces_4"
    /// Yukicoder: "yukicoder_1", "yukicoder_2", "yukicoder_3"
    /// Aoj: "aoj_volume1", "aoj_volume2", "aoj_joi_prelim_2023", ...
    /// YOJ: "yoj_graph", "yoj_math", "yoj_string", "yoj_datastructure"
    pub id: String,

    /// The naming convention for this field is <contest_id>
    /// Aoj: "volume1", "volume2", "joi_prelim_2023", ...
    pub raw_id: String,

    // The naming convention for this field is:
    // <contest_name>
    pub name: String,

    /// for example: this field is
    /// Atcoder: "abc", "arc", "agc", "other"
    /// Codeforces: "div.1", "div.2", "div.3", "div.4", "educational"
    /// Yukicoder: "normal", "other", "not-classified"
    /// Aoj: "volume 1", "volume 2", ..., "<largeCl>"
    /// Yosupo: "Graph", "Math", "String", "DataStructure", ...
    pub category: String,

    pub platform: Platform,

    pub phase: String,

    // contest start time in unix time
    pub start_time_seconds: Option<u64>,

    // contest duration in seconds
    pub duration_seconds: Option<u64>,

    pub url: String,

    pub problems: Vec<Problem>,
}

impl Contest {
    fn new(
        id: String,
        raw_id: String,
        name: String,
        category: String,
        platform: Platform,
        phase: String,
        start_time_seconds: Option<u64>,
        duration_seconds: Option<u64>,
        url: String,
        problems: Vec<Problem>,
    ) -> Self {
        Self {
            id,
            raw_id,
            name,
            category,
            platform,
            phase,
            start_time_seconds,
            duration_seconds,
            url,
            problems,
        }
    }

    /// Reconstructs a `Contest` struct from raw data obtained from an external API.
    ///
    /// This function is responsible for parsing the raw data specific to a given platform
    /// and converting it into a structured `Contest` object. Each platform may have a different
    /// data format, and this function handles those variations accordingly.
    pub fn reconstruct(
        raw_id: String,
        raw_name: String,
        platform: Platform,
        raw_phase: String,
        raw_start_time_seconds: Option<u64>,
        raw_duration_seconds: Option<u64>,
        problems: Vec<Problem>,
    ) -> Self {
        let (category, id, url, start_time_seconds, duration_seconds) = match platform {
            Platform::Atcoder => {
                let category = match raw_id {
                    id if id.starts_with("abc") => "ABC".to_string(),
                    id if id.starts_with("arc") => "ARC".to_string(),
                    id if id.starts_with("agc") => "AGC".to_string(),
                    id if id.starts_with("ahc") => "AHC".to_string(),
                    id if id.starts_with("past") => "PAST".to_string(),
                    id if id.starts_with("joi") => "JOI".to_string(),
                    id if id.starts_with("jag") => "JAG".to_string(),
                    _ => "Other".to_string(),
                };
                let id = format!("{}_{}", String::from(Platform::Atcoder), raw_id);
                let url = format!("https://atcoder.jp/contests/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (category, id, url, start_time_seconds, duration_seconds)
            }
            Platform::Codeforces => {
                let category = match raw_name {
                    raw_name if raw_name.contains("Div. 1 + Div. 2") => {
                        "Div. 1 + Div. 2".to_string()
                    }
                    raw_name if raw_name.contains("Div. 1") => "Div. 1".to_string(),
                    raw_name if raw_name.contains("Div. 2") => "Div. 2".to_string(),
                    raw_name if raw_name.contains("Div. 3") => "Div. 3".to_string(),
                    raw_name if raw_name.contains("Div. 4") => "Div. 4".to_string(),
                    raw_name if raw_name.contains("Global Round") => "Global Round".to_string(),
                    raw_name if raw_name.contains("Educational") => "Educational".to_string(),
                    raw_name if raw_name.contains("Kotlin") => "Kotlin".to_string(),
                    raw_name if raw_name.contains("ICPC") => "ICPC".to_string(),
                    raw_name if raw_name.contains("Q#") => "Q#".to_string(),
                    _ => "Other".to_string(),
                };
                let id = format!("{}_{}", String::from(Platform::Codeforces), raw_id);
                let url = format!("https://codeforces.com/contest/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (category, id, url, start_time_seconds, duration_seconds)
            }
            Platform::Yukicoder => {
                let category = if raw_name.starts_with("yukicoder contest") {
                    "Normal".to_string()
                } else {
                    "Other".to_string()
                };
                let id = format!("{}_{}", String::from(Platform::Yukicoder), raw_id);
                let url = format!("https://yukicoder.me/contests/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (category, id, url, start_time_seconds, duration_seconds)
            }
            Platform::Aoj => {
                let category = raw_id.split("_").collect::<Vec<&str>>()[0].to_string();
                let id = format!("{}_{}", String::from(Platform::Aoj), raw_name);
                let url = if raw_id.starts_with("volume") {
                    "https://onlinejudge.u-aizu.ac.jp/challenges/search/volumes".to_string()
                } else {
                    let [large_cl, middle_cl, year, ..] =
                        raw_id.split("_").collect::<Vec<&str>>().as_slice();
                    format!(
                        "https://onlinejudge.u-aizu.ac.jp/challenges/sources/{large_cl}/{middle_cl}?year={year}",
                    )
                };
                let start_time_seconds = None;
                let duration_seconds = None;

                (category, id, url, start_time_seconds, duration_seconds)
            }
            Platform::YOJ => {
                let category = raw_name;
                let id = format!("{}_{}", String::from(Platform::YOJ), raw_name);
                let url = "https://judge.yosupo.jp/".to_string();
                let start_time_seconds = None;
                let duration_seconds = None;

                (category, id, url, start_time_seconds, duration_seconds)
            }
        };

        Self {
            id,
            raw_id: raw_id.to_string(),
            name: raw_name.to_string(),
            category: category.to_string(),
            platform,
            phase: raw_phase.to_string(),
            start_time_seconds,
            duration_seconds,
            url,
            problems,
        }
    }
}
