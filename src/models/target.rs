use std::path::PathBuf;

use getset::Getters;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Getters, Debug, Deserialize, Serialize)]
#[get = "pub"]
pub struct Target {
    name: String,
    address: Url,
    identity: PathBuf,
}
