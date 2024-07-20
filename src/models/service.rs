use std::path::PathBuf;

use getset::Getters;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{outcome::Outcome, target::Target};

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
#[serde(tag = "type", content = "values")]
pub enum SourceType {
    Git { repo: Url },
    Tool { install: PathBuf },
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "values")]
pub enum ServiceKind {
    Node,
    Rust,
    Maven,
    External,
}

impl Service {
    pub fn install(&self, onto: &Target) -> anyhow::Result<Outcome> {
        // do cyclic dependency check here
        // if dependant service is not installed, install
        Ok(Outcome::Success)
    }
}
