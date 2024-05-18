use crate::domain::vo::category::{CodeforcesCategory, ContestCategory};

use super::external::CodeforcesContest;

pub fn classify_contest(contest: &CodeforcesContest) -> ContestCategory {
    // The order of the match arms is important for correct matching.
    match &contest.name {
        name if name.contains("Educational") => {
            ContestCategory::Codeforces(CodeforcesCategory::Educational)
        }
        name if name.contains("Global Round") => {
            ContestCategory::Codeforces(CodeforcesCategory::Global)
        }

        name if name.contains("Kotlin") => ContestCategory::Codeforces(CodeforcesCategory::Kotlin),
        name if name.contains("ICPC") => ContestCategory::Codeforces(CodeforcesCategory::ICPC),
        name if name.contains("Q#") => ContestCategory::Codeforces(CodeforcesCategory::QSharp),
        name if name.contains("Div. 1 + Div. 2") => {
            ContestCategory::Codeforces(CodeforcesCategory::Div1AndDiv2)
        }
        name if name.contains("Div. 1") => ContestCategory::Codeforces(CodeforcesCategory::Div1),
        name if name.contains("Div. 2") => ContestCategory::Codeforces(CodeforcesCategory::Div2),
        name if name.contains("Div. 3") => ContestCategory::Codeforces(CodeforcesCategory::Div3),
        name if name.contains("Div. 4") => ContestCategory::Codeforces(CodeforcesCategory::Div4),
        _ => ContestCategory::Codeforces(CodeforcesCategory::Other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_contest_with_name(name: &str) -> CodeforcesContest {
        CodeforcesContest {
            id: 1,
            name: name.to_string(),
            r#type: "Educational".to_string(),
            phase: "FINISHED".to_string(),
            frozen: false,
            duration_seconds: 7200,
            start_time_seconds: Some(1_000_000_000),
            relative_time_seconds: Some(0),
            prepared_by: None,
            description: None,
            difficulty: None,
            website_url: None,
            kind: None,
            icpc_region: None,
            country: None,
            city: None,
            season: None,
        }
    }

    #[test]
    fn test_classify_contest() {
        let contest =
            build_contest_with_name("Educational Codeforces Round 165 (Rated for Div. 2)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Educational)
        );

        let contest = build_contest_with_name("Codeforces Round #726 (Div. 2)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Div2)
        );

        let contest = build_contest_with_name("Codeforces Round #726 (Div. 1)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Div1)
        );

        let contest = build_contest_with_name("Codeforces Round #726 (Div. 3)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Div3)
        );

        let contest = build_contest_with_name("Codeforces Round #726 (Div. 4)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Div4)
        );

        let contest = build_contest_with_name("Codeforces Global Round 16");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Global)
        );

        let contest = build_contest_with_name("Codeforces Kotlin Heroes: Episode 8");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Kotlin)
        );

        let contest = build_contest_with_name("Codeforces ICPC Round #X");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::ICPC)
        );

        let contest = build_contest_with_name("Codeforces Round #726 (Div. 1 + Div. 2)");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Div1AndDiv2)
        );

        let contest = build_contest_with_name("Microsoft Q# Coding Contest - Summer 2020");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::QSharp)
        );

        let contest = build_contest_with_name("NOT_CLASSIFIED");
        assert_eq!(
            classify_contest(&contest),
            ContestCategory::Codeforces(CodeforcesCategory::Other)
        );
    }
}
