//! Classify yukicoder contest by their type.
//!
//! This utility function provides functionality to classify Yukicoder contests into categories

use crate::domain::vo::category::{ContestCategory, YukicoderCategory};

use super::external::YukicoderContest;

pub(super) fn classify_contest(contest: &YukicoderContest) -> ContestCategory {
    if contest.name.starts_with("yukicoder contest") {
        ContestCategory::Yukicoder(YukicoderCategory::Normal)
    } else {
        ContestCategory::Yukicoder(YukicoderCategory::Other)
    }
}
