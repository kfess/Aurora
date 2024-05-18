#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContestCategory {
    Atcoder(AtcoderCategory),
    Codeforces(CodeforcesCategory),
    Yukicoder(YukicoderCategory),
    YOJ(YOJCategory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtcoderCategory {
    ABC,
    ARC,
    AGC,
    AHC,
    PAST,
    JOI,
    JAG,
    ABCLike,
    ARCLike,
    AGCLike,
    Marathon,
    OtherSponsored,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeforcesCategory {
    Div1,
    Div2,
    Div3,
    Div4,
    Educational,
    Global,
    Kotlin,
    ICPC,
    QSharp,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YukicoderCategory {
    Normal,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YOJCategory {
    Sample,
    DataStructure,
    Graph,
    Tree,
    Math,
    Convolution,
    Polynomial,
    Matrix,
    String,
    Geometry,
    Other,
}

impl From<ContestCategory> for String {
    fn from(value: ContestCategory) -> Self {
        match value {
            ContestCategory::Atcoder(c) => match c {
                AtcoderCategory::ABC => "ABC".to_string(),
                AtcoderCategory::ARC => "ARC".to_string(),
                AtcoderCategory::AGC => "AGC".to_string(),
                AtcoderCategory::AHC => "AHC".to_string(),
                AtcoderCategory::PAST => "PAST".to_string(),
                AtcoderCategory::JOI => "JOI".to_string(),
                AtcoderCategory::JAG => "JAG".to_string(),
                AtcoderCategory::ABCLike => "ABC-Like".to_string(),
                AtcoderCategory::ARCLike => "ARC-Like".to_string(),
                AtcoderCategory::AGCLike => "AGC-Like".to_string(),
                AtcoderCategory::Marathon => "Marathon".to_string(),
                AtcoderCategory::OtherSponsored => "Other Sponsored".to_string(),
                AtcoderCategory::Other => "Other".to_string(),
            },
            ContestCategory::Codeforces(c) => match c {
                CodeforcesCategory::Div1 => "div. 1".to_string(),
                CodeforcesCategory::Div2 => "div. 2".to_string(),
                CodeforcesCategory::Div3 => "div. 3".to_string(),
                CodeforcesCategory::Div4 => "div. 4".to_string(),
                CodeforcesCategory::Educational => "Educational".to_string(),
                CodeforcesCategory::Global => "Global".to_string(),
                CodeforcesCategory::Kotlin => "Kotlin".to_string(),
                CodeforcesCategory::ICPC => "ICPC".to_string(),
                CodeforcesCategory::QSharp => "Q#".to_string(),
                CodeforcesCategory::Other => "Other".to_string(),
            },
            ContestCategory::Yukicoder(c) => match c {
                YukicoderCategory::Normal => "Normal".to_string(),
                YukicoderCategory::Other => "Other".to_string(),
            },
            ContestCategory::YOJ(c) => match c {
                YOJCategory::Sample => "Sample".to_string(),
                YOJCategory::DataStructure => "Data Structure".to_string(),
                YOJCategory::Graph => "Graph".to_string(),
                YOJCategory::Tree => "Tree".to_string(),
                YOJCategory::Math => "Math".to_string(),
                YOJCategory::Convolution => "Convolution".to_string(),
                YOJCategory::Polynomial => "Polynomial".to_string(),
                YOJCategory::Matrix => "Matrix".to_string(),
                YOJCategory::String => "String".to_string(),
                YOJCategory::Geometry => "Geometry".to_string(),
                YOJCategory::Other => "Other".to_string(),
            },
        }
    }
}
