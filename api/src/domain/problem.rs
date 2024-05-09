use super::value_object::platform::Platform;

#[derive(Clone, Debug)]
pub struct Problem {
    // The naming convention for this field is:
    // <platform>_<contest_name>_<problem_index>
    id: String,

    // The naming convention for this field is:
    // <platform>_<contest_name>
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

    // The value of this field is estimated as follows.
    // Implementing logistic regression with pairs of (x, y),
    // where 'x' represents the AtCoder internal rating and
    // 'y' indicates if the problem was solved within the time limit.
    // The 'Difficulty' is defined as the AtCoder internal rating value at which the probability of solving the problem within the time limit is 0.5.
    // This approach is similar to that used by AtCoder Problems.
    difficulty: Option<f64>,

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
            tags,
            url,
            solver_count,
            submissions,
            success_rate,
        }
    }

    pub fn reconstruct(
        contest_name: String,
        index: String,
        name: String,
        platform: Platform,
        raw_point: Option<f64>,
        difficulty: Option<f64>,
        tags: Vec<String>,
        url: String,
        solver_count: Option<u64>,
        submissions: Option<u64>,
    ) -> Self {
        let contest_id = format!("{}_{}", String::from(platform), contest_name);
        let id = format!("{}_{}_{}", String::from(platform), contest_name, index);
        let title = format!("{}. {}", index, name);

        let success_rate = match solver_count {
            Some(solver_count) => match submissions {
                Some(submissions) => Some(solver_count as f64 / submissions as f64 * 100.0),
                None => None,
            },
            None => None,
        };

        Self {
            id,
            contest_id,
            index,
            name,
            title,
            platform,
            raw_point,
            difficulty,
            tags,
            url,
            solver_count,
            submissions,
            success_rate,
        }
    }
}