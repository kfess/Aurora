use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Status {
    OK,
    FAILED,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblemAPIResponse {
    pub status: Status,
    pub result: Option<CodeforcesProblemResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Tag {
    Implementation,
    Math,
    Greedy,
    Dp,
    DataStructures,
    BruteForce,
    ConstructiveAlgorithms,
    Graph,
    Sortings,
    BinarySearch,
    DfsAndSimilar,
    Trees,
    Strings,
    NumberTheory,
    Combinatorics,
    Special,
    Geometry,
    Bitmasks,
    TwoPointers,
    Dsu,
    ShortestPaths,
    Probabilities,
    DivideAndConquer,
    Hashing,
    Games,
    Flows,
    Interactive,
    Martices,
    StringSuffixStructures,
    Fft,
    GraphMatchings,
    TernarySearch,
    ExpressionParsing,
    MeetInTheMiddle,
    TwoSat,
    ChineseRemainderTheorem,
    Schedules,
}

struct TagVisitor;

impl<'de> Visitor<'de> for TagVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a tag")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "implementation" => Ok(Tag::Implementation),
            "math" => Ok(Tag::Math),
            "greedy" => Ok(Tag::Greedy),
            "dp" => Ok(Tag::Dp),
            "data structures" => Ok(Tag::DataStructures),
            "brute force" => Ok(Tag::BruteForce),
            "constructive algorithms" => Ok(Tag::ConstructiveAlgorithms),
            "graphs" => Ok(Tag::Graph),
            "sortings" => Ok(Tag::Sortings),
            "binary search" => Ok(Tag::BinarySearch),
            "dfs and similar" => Ok(Tag::DfsAndSimilar),
            "trees" => Ok(Tag::Trees),
            "strings" => Ok(Tag::Strings),
            "number theory" => Ok(Tag::NumberTheory),
            "combinatorics" => Ok(Tag::Combinatorics),
            "*special" => Ok(Tag::Special),
            "geometry" => Ok(Tag::Geometry),
            "bitmasks" => Ok(Tag::Bitmasks),
            "two pointers" => Ok(Tag::TwoPointers),
            "dsu" => Ok(Tag::Dsu),
            "shortest paths" => Ok(Tag::ShortestPaths),
            "probabilities" => Ok(Tag::Probabilities),
            "divide and conquer" => Ok(Tag::DivideAndConquer),
            "hashing" => Ok(Tag::Hashing),
            "games" => Ok(Tag::Games),
            "flows" => Ok(Tag::Flows),
            "interactive" => Ok(Tag::Interactive),
            "matrices" => Ok(Tag::Martices),
            "string suffix structures" => Ok(Tag::StringSuffixStructures),
            "fft" => Ok(Tag::Fft),
            "graph matchings" => Ok(Tag::GraphMatchings),
            "ternary search" => Ok(Tag::TernarySearch),
            "expression parsing" => Ok(Tag::ExpressionParsing),
            "meet-in-the-middle" => Ok(Tag::MeetInTheMiddle),
            "2-sat" => Ok(Tag::TwoSat),
            "chinese remainder theorem" => Ok(Tag::ChineseRemainderTheorem),
            "schedules" => Ok(Tag::Schedules),
            _ => Err(de::Error::unknown_field(v, &["tag"])),
        }
    }
}

struct TagsVisitor;

impl<'de> Visitor<'de> for TagsVisitor {
    type Value = Vec<Tag>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of tag strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut tags = Vec::new();
        while let Some(tag_str) = seq.next_element::<String>()? {
            let tag = TagVisitor.visit_str(&tag_str)?;
            tags.push(tag);
        }
        Ok(tags)
    }
}

fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<Tag>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(TagsVisitor)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum ProblemType {
    PROGRAMMING,
    QUESTION,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblem {
    #[serde(rename = "contestId")]
    pub contest_id: u64,

    #[serde(rename = "problemsetName")]
    pub problemset_name: Option<String>,

    #[serde(rename = "index")]
    pub index: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub problem_type: ProblemType,

    #[serde(rename = "points")]
    pub points: Option<f64>,

    #[serde(rename = "rating")]
    pub rating: Option<u64>,

    #[serde(rename = "tags")]
    #[serde(deserialize_with = "deserialize_tags")]
    pub tags: Vec<Tag>,
    // pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblemStat {
    #[serde(rename = "contestId")]
    pub contest_id: u64,
    #[serde(rename = "index")]
    pub index: String,

    #[serde(rename = "solvedCount")]
    pub solved_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeforcesProblemResponse {
    pub problems: Vec<CodeforcesProblem>,

    #[serde(rename = "problemStatistics")]
    pub problem_statistics: Vec<CodeforcesProblemStat>,
}
