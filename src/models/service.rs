use std::path::PathBuf;

use getset::Getters;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Getters, Debug, Deserialize, Serialize)]
#[get = "pub"]
pub struct Service {
    name: String,
    source: SourceType,
    kind: ServiceKind,
    dependencies: Vec<PathBuf>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "values", rename_all = "lowercase")]
pub enum SourceType {
    Git { repo: Url, env: PathBuf, port: u32 },
    Tool { install: PathBuf },
}

impl SourceType {
    pub fn is_tool(&self) -> bool {
        matches!(self, SourceType::Tool { .. })
    }

    pub fn is_git(&self) -> bool {
        matches!(self, SourceType::Git { .. })
    }

    pub fn get_repo_url(&self) -> Option<&Url> {
        match self {
            Self::Git { repo, .. } => Some(repo),
            _ => None,
        }
    }

    pub fn get_port(&self) -> Option<u32> {
        match self {
            Self::Git { port, .. } => Some(*port),
            _ => None,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "values", rename_all = "snake_case")]
pub enum ServiceKind {
    Node,
    Rust,
    Maven,
    JavaJar,
}

impl ServiceKind {
    pub fn get_startup_cmd(&self) -> &'static str {
        match self {
            ServiceKind::Node => "node src/index.js",
            ServiceKind::Rust => "cargo run --release",
            ServiceKind::Maven => "mvn run",
            ServiceKind::JavaJar => "java --jar main.jar",
        }
    }
}
