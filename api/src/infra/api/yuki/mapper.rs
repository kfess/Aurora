    // use super::types::YukicoderContest;
// use crate::domain::{contest::Contest, problem::Problem, vo::platform::Platform};
// use crate::infra::api::yuki::types::YukicoderProblemWithStatistics;
// use crate::utils::format::num_to_alphabet;
// use chrono::DateTime;
// use std::collections::HashMap;

// // map problem id to contest names and problem index
// pub fn map_problem_ids(contests: &Vec<YukicoderContest>) -> HashMap<u64, (String, String)> {
//     contests
//         .iter()
//         .flat_map(|contest| {
//             contest
//                 .problem_id_list
//                 .iter()
//                 .enumerate()
//                 .map(move |(index, &problem_id)| {
//                     (problem_id, (contest.name.clone(), num_to_alphabet(index)))
//                 })
//         })
//         .collect()
// }

// // map contest id to problems
// pub fn map_contest_ids(contests: &Vec<YukicoderContest>) -> HashMap<u64, Vec<Problem>> {}

// pub fn build_problem(
//     contest_name: &str,
//     index: &str,
//     problem: &YukicoderProblemWithStatistics,
// ) -> Problem {
//     Problem::reconstruct(
//         contest_name.to_string(),
//         index.to_string(),
//         problem.title.to_string(),
//         Platform::Yukicoder,
//         Some(problem.level),
//         Option::None,
//         problem.tags.split(",").map(|s| s.to_string()).collect(),
//         format!("https://yukicoder.me/problems/no/{}", problem.no),
//         Some(problem.statistics.solved),
//         Some(problem.statistics.total),
//     )
// }

// pub fn build_contest(contest: &YukicoderContest, problems: Vec<Problem>) -> Contest {
//     let start_timestamp = DateTime::parse_from_rfc3339(&contest.date)
//         .unwrap()
//         .timestamp() as u64;

//     let duration_seconds = DateTime::parse_from_rfc3339(&contest.end_date)
//         .unwrap()
//         .timestamp() as u64
//         - start_timestamp;

//     Contest::reconstruct(
//         contest.name.to_string(),
//         Platform::Yukicoder,
//         "finished".to_string(),
//         Some(start_timestamp),
//         Some(duration_seconds),
//         format!("https://yukicoder.me/contests/{}", contest.id),
//         problems,
//     )
// }
