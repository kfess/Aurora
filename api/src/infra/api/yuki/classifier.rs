use crate::domain::vo::category::{ContestCategory, YukicoderCategory};

use super::types::YukicoderContest;

pub fn classify_contest(contest: &YukicoderContest) -> ContestCategory {
    if contest.name.starts_with("yukicoder contest") {
        ContestCategory::Yukicoder(YukicoderCategory::Normal)
    } else {
        ContestCategory::Yukicoder(YukicoderCategory::Other)
    }
}
