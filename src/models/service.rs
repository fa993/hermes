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
    Git { repo: Url, env: PathBuf },
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
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "values", rename_all = "lowercase")]
pub enum ServiceKind {
    Node,
    Rust,
    Maven,
    Java,
}
