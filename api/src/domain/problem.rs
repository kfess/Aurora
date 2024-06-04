use super::vo::platform::Platform;

#[derive(Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Problem {
    /// The unique identifier of the problem.
    ///
    /// For Atcoder, Codeforces and yukicoder, the `id` is formatted as `<platform>_<contest_id>_<index>`.
    /// - AtCoder: atcoder_abc100_A, atcoder_abc100_B, ...
    /// - Codeforces: codeforces_1000_A, codeforces_1000_B, ...
    /// - yukicoder: yukicoder_1000_A, yukicoder_1000_B, ...
    ///
    /// For YosupoOnlineJudge, the `id` is formatted as `<platform>_<category_name>_<index>`.
    /// - YosupoOnlineJudge: yosupo_graph_A, yosupo_math_A, ...
    ///
    /// For Aoj, the `id` is formatted as `<platform>_<volume_id>_<index>` or `<platform>_<large_cl>_<middle_cl>_<index>`.
    /// - Aoj: aoj_1_0001, aoj_1_0002, aoj_JOI_Prelim_0001, aoj_JOI_Prelim_0002, ...
    pub id: String,

    /// The contest ID of the problem.
    ///
    /// The value of this field is dependent on the platform.
    /// The common value is as follows:
    /// - AtCoder: <contest_id> (e.g., abc100)
    /// - Codeforces: <contest_id> (e.g., 1000)
    /// - yukicoder: <contest_id> (e.g., 1000)
    /// - YosupoOnlineJudge: <category_name> (e.g., Graph, Math, etc.)
    /// - Aoj: <volume_id> (e.g., 1) or <large_cl>_<middle_cl> (e.g., JOI_Prelim)
    pub contest_id: String,

    /// The index of the problem within the contest.
    ///
    /// The value of this field is dependent on the platform.
    /// The common value is as follows:
    /// - AtCoder: A, B, C, ...
    /// - Codeforces: A, B, C, ...
    /// - yukicoder: A, B, C, ...
    /// - YosupoOnlineJudge: A, B, C, ...
    /// - Aoj: "0001", "0002", "0003", ...
    pub index: String,

    /// The name of the problem.
    ///
    /// The value of this field is simply the name of the problem.
    /// The common value is as follows:
    /// - AtCoder: Two Anagrams
    /// - Codeforces: Tree with Maximum Cost
    /// - yukicoder: 2つの整数
    /// - YosupoOnlineJudge: Shortest Path
    /// - おせんべい
    pub name: String,

    /// The title of the problem.
    ///
    /// The title is formatted as `<index>. <name>`.
    /// - AtCoder: A. Two Anagrams
    /// - Codeforces: A. Tree with Maximum Cost
    /// - yukicoder: A. 2つの整数
    /// - YosupoOnlineJudge: A. Shortest Path
    /// - Aoj: 0001. おせんべい
    pub title: String,

    /// The platform to which the problem belongs.
    ///
    /// The value of this field is one of the following:
    /// - AtCoder
    /// - Codeforces
    /// - yukicoder
    /// - YosupoOnlineJudge
    /// - Aoj
    pub platform: Platform,

    /// The raw point of the problem.
    /// The value of this field is dependent on the platform.
    /// The common value is as follows:
    /// - AtCoder: 100, 200, 300, ..., 2000 (point)
    /// - Codeforces: 500, 1000, 1500, ..., 3500 (rating)
    /// - yukicoder: 0.5, 1, 1.5, ..., 4.5, 5 (level)
    /// - YosupoOnlineJudge: None
    /// - Aoj: None
    pub raw_point: Option<f64>,

    /// The value of this field is estimated using the following method:
    ///
    /// Logistic regression is performed on pairs of (x, y), where:
    /// - `x` represents the AtCoder internal rating
    /// - `y` indicates whether the problem was solved within the time limit
    ///
    /// The `Difficulty` is defined as the AtCoder internal rating at which the probability of solving the problem within the time limit is 0.5.
    /// This method is similar to the approach used by AtCoder Problems.
    pub difficulty: Option<f64>,

    /// Whether the estimated difficulty is experimental or not.
    ///
    /// If the difficulty is experimental, the value of this field is `true`.
    /// Otherwise, the value of this field is `false`.
    pub is_experimental: Option<bool>,

    /// The tags of the problem.
    pub tags: Vec<String>,

    /// The URL of the problem.
    ///
    /// The value of this field is dependent on the platform.
    /// The common value is as follows:
    /// - AtCoder: https://atcoder.jp/contests/abc100/tasks/abc100_a
    /// - Codeforces: https://codeforces.com/contest/1000/problem/A
    /// - yukicoder: https://yukicoder.me/problems/no/1000
    pub url: String,

    /// The number of users who have solved the problem.
    pub solver_count: Option<i64>,

    /// The number of submissions made to the problem.
    pub submissions: Option<i64>,

    /// The success rate of the problem.
    pub success_rate: Option<f64>,
}

impl Problem {
    pub fn new(
        id: String,
        contest_id: String,
        index: String,
        name: String,
        title: String,
        platform: Platform,
        raw_point: Option<f64>,
        difficulty: Option<f64>,
        is_experimental: Option<bool>,
        tags: Vec<String>,
        url: String,
        solver_count: Option<i64>,
        submissions: Option<i64>,
        success_rate: Option<f64>,
    ) -> Self {
        Self {
            id,
            contest_id,
            index,
            name,
            title,
            platform,
            raw_point,
            difficulty,
            is_experimental,
            tags,
            url,
            solver_count,
            submissions,
            success_rate,
        }
    }

    pub fn reconstruct(
        platform: Platform,
        raw_contest_id: &str,
        raw_index: &str,
        raw_name: &str,
        raw_point: Option<f64>,
        difficulty: Option<f64>,
        is_experimental: Option<bool>,
        raw_tags: Vec<String>,
        raw_url: &str,
        raw_solver_count: Option<i64>,
        raw_submissions: Option<i64>,
    ) -> Self {
        let id = format!(
            "{}_{}_{}",
            String::from(platform),
            raw_contest_id,
            raw_index
        );
        let title = format!("{raw_index}. {raw_name}");

        let success_rate = match raw_solver_count {
            Some(solver_count) => match raw_submissions {
                Some(submissions) => Some(solver_count as f64 / submissions as f64 * 100.0),
                None => None,
            },
            None => None,
        };

        Self {
            id,
            contest_id: String::from(raw_contest_id),
            index: String::from(raw_index),
            name: String::from(raw_name),
            title,
            platform,
            raw_point,
            difficulty,
            is_experimental,
            tags: raw_tags,
            url: String::from(raw_url),
            solver_count: raw_solver_count,
            submissions: raw_submissions,
            success_rate,
        }
    }
}
