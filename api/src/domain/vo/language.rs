use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    CPP,
    Python,
    Java,
    C,
    CSharp,
    Kotlin,
    JavaScript,
    TypeScript,
    Ruby,
    Go,
    Rust,
    Swift,
    Haskell,
    Scala,
    PHP,
    Perl,
    FPC,
    OCaml,
    Bash,
    Lua,
    NodeJS,
    D,
    Nim,
    Crystal,
    Ada,
    Delphi,
    R,
    Tcl,
    Pike,
    PascalABCNET,
    Picat,
    Factor,
    Cobol,
    Befunge,
    Io,
    J,
    QSharp,
    Roco,
    PlainText,
    Text,
    SQL,
    Other,
}

// To optimize the match process,
// the languages in the match sequence are ordered
// by their prevalence in usage.
impl From<&str> for Language {
    fn from(lang: &str) -> Self {
        match Regex::new(r"\s+\(.*\)")
            .unwrap()
            .replace_all(lang, "")
            .to_lowercase()
            .as_str()
        {
            lang if lang.contains("c++") | lang.contains("cpp") | lang.contains("clang++") => {
                Language::CPP
            }
            lang if lang.contains("python") || lang.contains("pypy") => Language::Python,
            lang if lang.contains("java") && !lang.contains("javascript") => Language::Java,
            lang if lang.contains("c#") => Language::CSharp,
            lang if lang.contains("rust") => Language::Rust,
            lang if lang.contains("go") => Language::Go,
            lang if lang.contains("kotlin") => Language::Kotlin,
            lang if lang.contains("javascript") => Language::JavaScript,
            lang if lang.contains("typescript") => Language::TypeScript,
            lang if lang == "c" || lang.contains("gnu c11") => Language::C,
            lang if lang.contains("ruby") => Language::Ruby,
            lang if lang.contains("swift") => Language::Swift,
            lang if lang.contains("haskell") => Language::Haskell,
            lang if lang.contains("scala") => Language::Scala,
            lang if lang.contains("php") => Language::PHP,
            lang if lang.contains("perl") => Language::Perl,
            lang if lang.contains("fpc") => Language::FPC,
            lang if lang.contains("ocaml") => Language::OCaml,
            lang if lang.contains("bash") => Language::Bash,
            lang if lang.contains("lua") => Language::Lua,
            lang if lang.contains("node") => Language::NodeJS,
            lang if lang.contains("nim") => Language::Nim,
            lang if lang.contains("crystal") => Language::Crystal,
            lang if lang.contains("ada") => Language::Ada,
            lang if lang.contains("delphi") => Language::Delphi,
            lang if lang.starts_with("d") => Language::D, // this is placed after "delphi"
            lang if lang == "r" => Language::R,
            lang if lang.contains("tcl") => Language::Tcl,
            lang if lang.contains("pike") => Language::Pike,
            lang if lang.contains("pascalabc") => Language::PascalABCNET,
            lang if lang.contains("picat") => Language::Picat,
            lang if lang.contains("factor") => Language::Factor,
            lang if lang.contains("cobol") => Language::Cobol,
            lang if lang.contains("befunge") => Language::Befunge,
            lang if lang.contains("io") => Language::Io,
            lang if lang == "j" => Language::J,
            lang if lang.contains("q#") || lang == "qsharp" => Language::QSharp,
            lang if lang.contains("roco") => Language::Roco,
            lang if lang.contains("plain") => Language::PlainText,
            lang if lang.contains("text") => Language::Text,
            lang if lang.contains("sql") => Language::SQL,
            _ => Language::Other,
        }
    }
}

impl From<Language> for String {
    fn from(value: Language) -> Self {
        match value {
            Language::CPP => "C++".to_string(),
            Language::Python => "Python".to_string(),
            Language::Java => "Java".to_string(),
            Language::C => "C".to_string(),
            Language::Kotlin => "Kotlin".to_string(),
            Language::JavaScript => "JavaScript".to_string(),
            Language::TypeScript => "TypeScript".to_string(),
            Language::Ruby => "Ruby".to_string(),
            Language::Go => "Go".to_string(),
            Language::Rust => "Rust".to_string(),
            Language::Swift => "Swift".to_string(),
            Language::Haskell => "Haskell".to_string(),
            Language::Scala => "Scala".to_string(),
            Language::PHP => "PHP".to_string(),
            Language::Perl => "Perl".to_string(),
            Language::CSharp => "C#".to_string(),
            Language::FPC => "FPC".to_string(),
            Language::OCaml => "OCaml".to_string(),
            Language::Bash => "Bash".to_string(),
            Language::Lua => "Lua".to_string(),
            Language::NodeJS => "Node.js".to_string(),
            Language::D => "D".to_string(),
            Language::Nim => "Nim".to_string(),
            Language::Crystal => "Crystal".to_string(),
            Language::Ada => "Ada".to_string(),
            Language::Delphi => "Delphi".to_string(),
            Language::R => "R".to_string(),
            Language::Tcl => "Tcl".to_string(),
            Language::Pike => "Pike".to_string(),
            Language::PascalABCNET => "PascalABC.NET".to_string(),
            Language::Picat => "Picat".to_string(),
            Language::Factor => "Factor".to_string(),
            Language::Cobol => "Cobol".to_string(),
            Language::Befunge => "Befunge".to_string(),
            Language::Io => "Io".to_string(),
            Language::J => "J".to_string(),
            Language::QSharp => "Q#".to_string(),
            Language::Roco => "Roco".to_string(),
            Language::PlainText => "Plain Text".to_string(),
            Language::Text => "Text".to_string(),
            Language::SQL => "SQL".to_string(),
            Language::Other => "Other".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for v in ["C++", "C++ 20 (gcc 12.2)", "cpp", "clang++"] {
            assert_eq!(Language::from(v), Language::CPP);
        }

        for v in ["Python", "PyPy", "Python (CPython 3.11.4)"] {
            assert_eq!(Language::from(v), Language::Python);
        }

        for v in ["Java", "Java (OpenJDK 11.0.12)", "Java (OpenJDK 17)"] {
            assert_eq!(Language::from(v), Language::Java);
        }

        for v in ["C#", "C# 11.0 (.NET 7.0.7)", "C# 11.0 AOT (.NET 7.0.7)"] {
            assert_eq!(Language::from(v), Language::CSharp);
        }

        for v in ["Rust", "Rust (rustc 1.70.0)"] {
            assert_eq!(Language::from(v), Language::Rust);
        }

        for v in ["Go", "Go (1.20.6)"] {
            assert_eq!(Language::from(v), Language::Go);
        }

        for v in ["Kotlin", "Kotlin (kotlin/JVM 1.8.20)"] {
            assert_eq!(Language::from(v), Language::Kotlin);
        }

        for v in ["JavaScript", "JavaScript (Node.js 18.16.1)"] {
            assert_eq!(Language::from(v), Language::JavaScript);
        }

        for v in ["TypeScript", "TypeScript (Deno 1.35.1)"] {
            assert_eq!(Language::from(v), Language::TypeScript);
        }

        for v in ["C", "C (Clang 13.0.0)", "C (gcc 12.2.0)"] {
            assert_eq!(Language::from(v), Language::C);
        }
    }
}
