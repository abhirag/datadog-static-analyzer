use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Debug, Serialize, Eq, PartialEq)]
pub enum OutputFormat {
    Json,
    Sarif,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "JSON",
            Self::Sarif => "SARIF",
        };
        write!(f, "{s}")
    }
}

#[derive(Copy, Clone, Deserialize, Debug, Serialize, Eq, Hash, PartialEq)]
pub enum Language {
    #[serde(rename = "CSHARP")]
    Csharp,
    #[serde(rename = "DOCKERFILE")]
    Dockerfile,
    #[serde(rename = "GO")]
    Go,
    #[serde(rename = "JAVA")]
    Java,
    #[serde(rename = "JAVASCRIPT")]
    JavaScript,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "PYTHON")]
    Python,
    #[serde(rename = "RUST")]
    Rust,
    #[serde(rename = "TYPESCRIPT")]
    TypeScript,
    #[serde(rename = "YAML")]
    Yaml,
}

#[allow(dead_code)]
static ALL_LANGUAGES: &[Language] = &[
    Language::JavaScript,
    Language::Python,
    Language::Rust,
    Language::TypeScript,
];

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Csharp => "C#",
            Self::Dockerfile => "Dockerfile",
            Self::Go => "Go",
            Self::Java => "java",
            Self::JavaScript => "javascript",
            Self::Json => "json",
            Self::Python => "python",
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::Yaml => "yaml",
        };
        write!(f, "{s}")
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, Builder)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "position (line: {}, col: {})", self.line, self.col)
    }
}
