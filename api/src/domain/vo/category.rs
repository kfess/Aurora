pub enum ContestCategory {
    Atcoder(AtcoderCategory),
    Codeforces(CodeforcesCategory),
    Yukicoder(YukicoderCategory),
}

enum AtcoderCategory {
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

enum YukicoderCategory {
    Normal,
    Other,
}
