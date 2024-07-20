use std::{fs::read_to_string, path::PathBuf};

use anyhow::Context;
use log::info;

use crate::models::{service::Service, target::Target};

pub async fn push(service_path: PathBuf, target_path: PathBuf) -> anyhow::Result<()> {
    info!("Getting service details {}", service_path.display());
    let file_content = read_to_string(service_path.as_path())?;
    let service: Service = toml::from_str(&file_content).context(format!(
        "Failed to parse toml for service file {}",
        service_path.display()
    ))?;
    info!("Getting target details {}", target_path.display());
    let file_content = read_to_string(target_path.as_path())?;
    let target: Target = toml::from_str(&file_content).context(format!(
        "Failed to parse toml for target file {}",
        target_path.display()
    ))?;
    info!(
        "Pushing service {} to target {}",
        service.name(),
        target.name
    );
    target.push(&service).await
}
