use super::vo::platform::Platform;

#[derive(Clone, Debug, PartialEq)]
pub struct Problem {
    // The naming convention for this field is:
    // <platform>_<raw_id>_<problem_index>
    id: String,

    // The naming convention for this field is:
    // <platform>_<raw_id>
    contest_id: String,

    // A, B, C, ... or 1, 2, 3, ...
    index: String,

    name: String,

    // <problem_index>. <problem_name>
    title: String,

    platform: Platform,

    // The value of this field is dependent on the platform.
    // The common value is as follows:
    // - AtCoder: 100, 200, 300, ..., 2000 (point)
    // - Codeforces: 500, 1000, 1500, ..., 3500 (rating)
    // - yukicoder: 0.5, 1, 1.5, ..., 4.5, 5 (level)
    // - YosupoOnlineJudge: None
    // - Aoj: None
    raw_point: Option<f64>,

    /// The value of this field is estimated using the following method:
    ///
    /// Logistic regression is performed on pairs of (x, y), where:
    /// - `x` represents the AtCoder internal rating
    /// - `y` indicates whether the problem was solved within the time limit
    ///
    /// The `Difficulty` is defined as the AtCoder internal rating at which the probability of solving the problem within the time limit is 0.5.
    /// This method is similar to the approach used by AtCoder Problems.
    difficulty: Option<f64>,

    /// Whether the estimated difficulty is experimental or not.
    ///
    /// If the difficulty is experimental, the value of this field is `true`.
    /// Otherwise, the value of this field is `false`.
    is_experimental: Option<bool>,

    tags: Vec<String>,

    url: String,

    solver_count: Option<u64>,

    submissions: Option<u64>,

    success_rate: Option<f64>,
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
        solver_count: Option<u64>,
        submissions: Option<u64>,
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
        raw_contest_id: String,
        raw_index: String,
        raw_name: String,
        platform: Platform,
        raw_point: Option<f64>,
        difficulty: Option<f64>,
        is_experimental: Option<bool>,
        raw_tags: Vec<String>,
        raw_url: String,
        raw_solver_count: Option<u64>,
        raw_submissions: Option<u64>,
    ) -> Self {
        let contest_id = format!("{}_{}", String::from(platform), raw_contest_id);
        let id = format!(
            "{}_{}_{}",
            String::from(platform),
            raw_contest_id,
            raw_index
        );
        let title = format!("{}. {}", raw_index, raw_name);

        let success_rate = match raw_solver_count {
            Some(solver_count) => match raw_submissions {
                Some(submissions) => Some(solver_count as f64 / submissions as f64 * 100.0),
                None => None,
            },
            None => None,
        };

        Self {
            id,
            contest_id,
            index: raw_index,
            name: raw_name,
            title,
            platform,
            raw_point,
            difficulty,
            is_experimental,
            tags: raw_tags,
            url: raw_url,
            solver_count: raw_solver_count,
            submissions: raw_submissions,
            success_rate,
        }
    }
}
