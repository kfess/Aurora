use crate::domain::vo::category::{ContestCategory, YOJCategory};

pub(super) fn classify_contest(category_name: &str) -> ContestCategory {
    match category_name {
        "Sample" => ContestCategory::YOJ(YOJCategory::Sample),
        "Data Structure" => ContestCategory::YOJ(YOJCategory::DataStructure),
        "Graph" => ContestCategory::YOJ(YOJCategory::Graph),
        "Tree" => ContestCategory::YOJ(YOJCategory::Tree),
        "Math" => ContestCategory::YOJ(YOJCategory::Math),
        "Convolution" => ContestCategory::YOJ(YOJCategory::Convolution),
        "Polynomial" => ContestCategory::YOJ(YOJCategory::Polynomial),
        "Matrix" => ContestCategory::YOJ(YOJCategory::Matrix),
        "String" => ContestCategory::YOJ(YOJCategory::String),
        "Geometry" => ContestCategory::YOJ(YOJCategory::Geometry),
        _ => ContestCategory::YOJ(YOJCategory::Other),
    }
}
