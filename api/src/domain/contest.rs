use super::problem::Problem;
use super::vo::platform::Platform;

/// Represents a programming contest with details specific to a competition platform.
///
/// This struct includes all necessary information about a contest such as its unique identifier,
/// raw ID as per the external API, contest name, category based on the type of contest,
/// the platform from which the contest information is sourced, current phase of the contest,
/// and timing details like start time and duration.
///
/// # Fields
/// - `id`: A unique identifier for the contest, usually prefixed with the platform name.
/// - `raw_id`: The identifier used by the external API, without platform prefix.
/// - `name`: The official name of the contest.
/// - `category`: A classification of the contest (e.g., division, type) based on its characteristics.
/// - `platform`: The platform (`Platform` enum) that the contest information pertains to.
/// - `phase`: Current phase of the contest (e.g., "upcoming", "live", "completed").
/// - `start_time_seconds`: Optional start time of the contest in Unix time seconds.
/// - `duration_seconds`: Optional duration of the contest in seconds.
/// - `url`: URL to the contest page on the corresponding platform's website.
/// - `problems`: A vector of `Problem` objects associated with the contest.
///
#[derive(Clone, Debug)]
pub struct Contest {
    /// A globally unique identifier for the contest.
    ///
    /// The `id` is typically formatted as `<platform>_<raw_id>`.
    /// - Atcoder: "atcoder_abc001", "atcoder_arc001", etc.
    /// - Codeforces: "codeforces_1", "codeforces_2", etc.
    /// - Yukicoder: "yukicoder_1", "yukicoder_2", etc.
    /// - Aoj: "aoj_volume1", "aoj_volume2", "aoj_joi_prelim_2023", etc.
    /// - YOJ: "yoj_graph", "yoj_math", etc.
    id: String,

    /// The platform-specific identifier of the contest. This is part of the `id`.
    /// - Atcoder: "abc001", "arc001", "agc001", ...
    /// - Codeforces: "1", "2", "3", "4"
    /// - Yukicoder: "1", "2", "3"
    /// - Aoj: "volume_1", "volume_2", "joi_prelim_2023", ...
    /// - YOJ: "graph", "math", "string", "datastructure"
    raw_id: String,

    /// The official name of the contest as it appears on the platform.
    /// - Atcoder: "AtCoder Beginner Contest 001", "AtCoder Regular Contest 001", ...
    /// - Codeforces: "Codeforces Round #1", "Codeforces Round #2", ...
    /// - Yukicoder: "Yukicoder Contest 1", "Yukicoder Contest 2", ...
    /// - Aoj: "Volume 1", "Volume 2", "22nd Japanese Olympiad in Informatics, Preliminary Round 1-1", ...
    /// - YOJ: "Graph", "Math", ...
    name: String,

    /// A classification of the contest based on its characteristics.
    /// - Atcoder: "ABC", "ARC", "AGC", "AHC", "PAST", "JOI", "JAG", "ABCLike", "ARCLike", "AGCLike", "Marathon", "OtherSponsored", "Other"
    /// - Codeforces: "Div. 1", "Div. 2", "Div. 3", "Div. 4", "Educational", "Global Round", "Kotlin", "ICPC", "Q#", "Other"
    /// - Yukicoder: "Normal", "Other", "Not-classified"
    /// - Aoj: "Volume 1", "Volume 2", "JOI", etc.
    /// - YOJ: "Graph", "Math", "String", "DataStructure", "Geometry", etc.
    category: String,

    /// The platform where the contest is hosted.
    platform: Platform,

    /// The current phase of the contest.
    /// - All Platforms: "before", "coding", "finished", "unknown"
    phase: String,

    /// The start time of the contest in Unix time seconds, if known.
    /// - This is optional and may not be available for all contests on all platforms.
    start_time_seconds: Option<u64>,

    /// The duration of the contest in seconds, if known.
    /// - This is optional and varies greatly depending on the contest format and platform.
    duration_seconds: Option<u64>,

    /// The URL to the contest's page on the hosting platform's website.
    /// - Atcoder: "https://atcoder.jp/contests/abc001"
    /// - Codeforces: "https://codeforces.com/contest/1"
    /// - Yukicoder: "https://yukicoder.me/contests/1"
    /// - Aoj: "https://onlinejudge.u-aizu.ac.jp/challenges/search/volumes/1"
    /// - YOJ: "https://judge.yosupo.jp/"
    url: String,

    // A list of problems associated with the contest.
    ///
    /// This includes all problems that are part of the contest, each represented by a `Problem` struct.
    problems: Vec<Problem>,
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
        category: String,
        platform: Platform,
        raw_phase: String,
        raw_start_time_seconds: Option<u64>,
        raw_duration_seconds: Option<u64>,
        problems: Vec<Problem>,
    ) -> Self {
        let (id, url, start_time_seconds, duration_seconds) = match platform {
            Platform::Atcoder => {
                let id = format!("{}_{}", String::from(Platform::Atcoder), raw_id);
                let url = format!("https://atcoder.jp/contests/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (id, url, start_time_seconds, duration_seconds)
            }
            Platform::Codeforces => {
                let id = format!("{}_{}", String::from(Platform::Codeforces), raw_id);
                let url = format!("https://codeforces.com/contest/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (id, url, start_time_seconds, duration_seconds)
            }
            Platform::Yukicoder => {
                let id = format!("{}_{}", String::from(Platform::Yukicoder), raw_id);
                let url = format!("https://yukicoder.me/contests/{}", raw_id);
                let start_time_seconds = raw_start_time_seconds;
                let duration_seconds = raw_duration_seconds;

                (id, url, start_time_seconds, duration_seconds)
            }
            Platform::Aoj => {
                // let category = raw_id.split("_").collect::<Vec<&str>>()[0].to_string();
                let id = format!("{}_{}", String::from(Platform::Aoj), raw_name);
                let url = if raw_id.starts_with("volume") {
                    "https://onlinejudge.u-aizu.ac.jp/challenges/search/volumes".to_string()
                } else {
                    match raw_id.split("_").collect::<Vec<&str>>().as_slice() {
                        [large_cl, middle_cl, year] => {
                            format!(
                                "https://onlinejudge.u-aizu.ac.jp/challenges/sources/{large_cl}/{middle_cl}?year={year}",
                            )
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                };
                let start_time_seconds = None;
                let duration_seconds = None;

                (id, url, start_time_seconds, duration_seconds)
            }
            Platform::YOJ => {
                let id = format!("{}_{}", String::from(Platform::YOJ), raw_name);
                let url = "https://judge.yosupo.jp/".to_string();
                let start_time_seconds = None;
                let duration_seconds = None;

                (id, url, start_time_seconds, duration_seconds)
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
