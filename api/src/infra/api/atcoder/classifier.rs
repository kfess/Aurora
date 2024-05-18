//! Classifies AtCoder contests by their type.
//!
//! This utility function provides functionality to classify AtCoder contests into categories
//! such as ABC (AtCoder Beginner Contest), ARC (AtCoder Regular Contest), and AGC (AtCoder Grand Contest).
//! Each contest type may have specific characteristics and purposes. For example:
//! - **ABC**: Aimed at beginners, typically features easier problems.
//! - **ARC**: Intermediate level contests with a mix of problem difficulties.
//! - **AGC**: Advanced level contests with more challenging problems.
//!

use super::types::AtcoderContest;
use crate::domain::vo::category::{AtcoderCategory, ContestCategory};
use regex::Regex;

/// The start time of the first AtCoder Grand Contest (AGC).
/// This is used to determine if a contest is rated or not.
/// If the contest starts before this time, it is considered unrated.
/// Otherwise, it is considered rated.
const AGC_001_START: u64 = 1_468_670_400;

/// Special heuristic contests that do not follow the usual naming convention.
/// So, we need to handle them separately.
/// if you want to add a new special contest, add its id here.
const AHC_SPECIAL_CONTESTS: [&'static str; 1] = ["toyota2023summer-final"];

const MAYBE_MARATHON_NAME_REGEX: &'static str =
    r"(^Chokudai Contest|ハーフマラソン|^HACK TO THE FUTURE|Asprova|Heuristics Contest)";

const MAYBE_MARATHON_ID_REGEX: &'static str =
    r"(^future-meets-you-contest|^hokudai-hitachi|^toyota-hc)";

const MAYBE_MARATHON_IDS: [&'static str; 7] = [
    "toyota2023summer-final-open",
    "genocon2021",
    "stage0-2021",
    "caddi2019",
    "pakencamp-2019-day2",
    "kuronekoyamato-contest2019",
    "wn2017_1",
];

const MAYBE_OTHER_SPONSORED_NAMES: &'static str = r"ドワンゴ|^Mujin|SoundHound|^codeFlyer|^COLOCON|みんなのプロコン|CODE THANKS FESTIVAL|CODE FESTIVAL|^DISCO|日本最強プログラマー学生選手権|全国統一プログラミング王|Indeed|^Donuts|^dwango|^DigitalArts|^Code Formula|天下一プログラマーコンテスト|^Toyota";

/// Classifies an AtCoder contest into a category.
/// This function uses various heuristics to determine the category of an AtCoder contest.
/// The category is returned as a `ContestCategory` enum.
/// # Arguments
/// - `contest`: The AtCoder contest to classify.
/// # Returns
/// The category of the contest as a `ContestCategory` enum.
pub fn classify_contest(contest: &AtcoderContest) -> ContestCategory {
    if Regex::new(r"^abc\d{3,}").unwrap().is_match(&contest.id) {
        return ContestCategory::Atcoder(AtcoderCategory::ABC);
    }

    if Regex::new(r"^arc\d{3,}").unwrap().is_match(&contest.id) {
        return ContestCategory::Atcoder(AtcoderCategory::ARC);
    }

    if Regex::new(r"^agc\d{3,}").unwrap().is_match(&contest.id) {
        return ContestCategory::Atcoder(AtcoderCategory::AGC);
    }

    if Regex::new(r"^ahc\d{3,}").unwrap().is_match(&contest.id)
        || AHC_SPECIAL_CONTESTS.contains(&contest.id.as_str())
    {
        return ContestCategory::Atcoder(AtcoderCategory::AHC);
    }

    if is_rated_contest(contest, 100) {
        return classifiy_other_rated_contest(contest);
    }

    if contest.id.starts_with("past") {
        return ContestCategory::Atcoder(AtcoderCategory::PAST);
    }

    if contest.id.starts_with("joi") {
        return ContestCategory::Atcoder(AtcoderCategory::JOI);
    }

    if Regex::new(r"^(jag|JAG)").unwrap().is_match(&contest.id) {
        return ContestCategory::Atcoder(AtcoderCategory::JAG);
    }

    if Regex::new(MAYBE_MARATHON_NAME_REGEX)
        .unwrap()
        .is_match(&contest.title)
        || Regex::new(MAYBE_MARATHON_ID_REGEX)
            .unwrap()
            .is_match(&contest.id)
        || MAYBE_MARATHON_IDS.contains(&contest.id.as_str())
    {
        return ContestCategory::Atcoder(AtcoderCategory::Marathon);
    }

    if Regex::new(&MAYBE_OTHER_SPONSORED_NAMES)
        .unwrap()
        .is_match(&contest.title)
    {
        return ContestCategory::Atcoder(AtcoderCategory::OtherSponsored);
    }

    return ContestCategory::Atcoder(AtcoderCategory::Other);
}

fn is_rated_contest(contest: &AtcoderContest, problem_count: u64) -> bool {
    contest.rate_change != "-" && contest.start_epoch_second >= AGC_001_START && problem_count > 2
}

fn classifiy_other_rated_contest(contest: &AtcoderContest) -> ContestCategory {
    match get_rated_target(&contest) {
        Target::ABC => ContestCategory::Atcoder(AtcoderCategory::ABCLike),
        Target::ARC => ContestCategory::Atcoder(AtcoderCategory::ARCLike),
        Target::AGC => ContestCategory::Atcoder(AtcoderCategory::AGCLike),
        Target::Unrated => {
            unreachable!();
        }
    }
}

enum Target {
    ABC,
    ARC,
    AGC,
    Unrated,
}

fn get_rated_target(contest: &AtcoderContest) -> Target {
    if AGC_001_START > contest.start_epoch_second {
        return Target::Unrated;
    }

    match contest.rate_change.as_str() {
        "-" => Target::Unrated,
        "All" => Target::AGC,
        _ => {
            let range = contest
                .rate_change
                .split("~")
                .map(|s| s.trim())
                .collect::<Vec<&str>>();

            if range.len() != 2 {
                return Target::Unrated;
            }

            match range.get(1) {
                Some(&val) => {
                    if val.parse::<u64>().unwrap() < 2000 {
                        return Target::ABC;
                    } else {
                        return Target::ARC;
                    }
                }
                None => {
                    if range.get(0).is_some() {
                        return Target::AGC;
                    }
                    return Target::Unrated;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_contest_with_id(id: &str) -> AtcoderContest {
        AtcoderContest {
            id: id.to_string(),
            ..Default::default()
        }
    }

    fn build_contest_with_title(title: &str) -> AtcoderContest {
        AtcoderContest {
            title: title.to_string(),
            ..Default::default()
        }
    }

    fn build_contest_with_rate_change(rate_change: &str) -> AtcoderContest {
        AtcoderContest {
            rate_change: rate_change.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_abc() {
        let contest = build_contest_with_id("abc001");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ABC)
        );

        let contest = build_contest_with_id("abc1000");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ABC)
        );
    }

    #[test]
    fn test_arc() {
        let contest = build_contest_with_id("arc001");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ARC)
        );

        let contest = build_contest_with_id("arc1000");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ARC)
        );
    }

    #[test]
    fn test_agc() {
        let contest = build_contest_with_id("agc001");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::AGC)
        );

        let contest = build_contest_with_id("agc1000");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::AGC)
        );
    }

    #[test]
    fn test_ahc() {
        let contest = build_contest_with_id("ahc001");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::AHC)
        );

        let contest = build_contest_with_id("ahc1000");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::AHC)
        );

        // Special contests
        for id in AHC_SPECIAL_CONTESTS.iter() {
            let contest = build_contest_with_id(id);
            assert_eq!(
                classify_contest(&contest),
                ContestCategory::Atcoder(AtcoderCategory::AHC)
            );
        }
    }

    #[test]
    fn test_other_rated() {
        let contest = build_contest_with_rate_change(" ~ 1999");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ABCLike)
        );

        let contest = build_contest_with_rate_change(" ~ 2799");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::ARCLike)
        );

        let contest = build_contest_with_rate_change("All");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::AGCLike)
        );
    }

    #[test]
    fn test_past() {
        let contest = build_contest_with_id("past15-open");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::PAST)
        );
    }

    #[test]
    fn test_joi() {
        let contest = build_contest_with_id("joi2006yo");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::JOI)
        );
    }

    #[test]
    fn test_jag() {
        let contest = build_contest_with_id("jag2015summer-day2");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::JAG)
        );

        let contest = build_contest_with_id("JAG2015summer-day2");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::JAG)
        );
    }

    #[test]
    fn test_marathon() {
        let titles: [&str; 5] = [
            "Chokudai Contest",
            "ハーフマラソン",
            "HACK TO THE FUTURE",
            "Asprova",
            "Heuristics Contest",
        ];
        for title in titles {
            let contest = build_contest_with_title(title);
            assert_eq!(
                classify_contest(&contest),
                ContestCategory::Atcoder(AtcoderCategory::Marathon)
            );
        }

        let ids: [&str; 7] = [
            "future-meets-you-contest",
            "hokudai-hitachi",
            "toyota-hc",
            "genocon2021",
            "stage0-2021",
            "caddi2019",
            "pakencamp-2019-day2",
        ];
        for id in ids {
            let contest = build_contest_with_id(id);
            assert_eq!(
                classify_contest(&contest),
                ContestCategory::Atcoder(AtcoderCategory::Marathon)
            );
        }
    }

    fn test_other_sponsored() {
        let titles: [&str; 18] = [
            "ドワンゴ",
            "Mujin",
            "SoundHound",
            "codeFlyer",
            "COLOCON",
            "みんなのプロコン",
            "CODE THANKS FESTIVAL",
            "CODE FESTIVAL",
            "DISCO",
            "日本最強プログラマー学生選手権",
            "全国統一プログラミング王",
            "Indeed",
            "Donuts",
            "dwango",
            "DigitalArts",
            "Code Formula",
            "天下一プログラマーコンテスト",
            "Toyota",
        ];

        for title in titles {
            let contest = build_contest_with_title(title);
            assert_eq!(
                classify_contest(&contest),
                ContestCategory::Atcoder(AtcoderCategory::OtherSponsored)
            );
        }
    }

    #[test]
    fn test_other() {
        let contest = build_contest_with_id("other");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Atcoder(AtcoderCategory::Other)
        );

        test_other_sponsored();
    }
}
