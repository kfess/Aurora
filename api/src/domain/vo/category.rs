#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContestCategory {
    Atcoder(AtcoderCategory),
    Codeforces(CodeforcesCategory),
    Yukicoder(YukicoderCategory),
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
enum CodeforcesCategory {
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
enum YukicoderCategory {
    Normal,
    Other,
}
