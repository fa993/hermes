use std::{fs::read_to_string, path::Path};

use anyhow::Context;
use log::info;

use crate::models::{service::Service, target::Target};

pub fn parse_args<T: AsRef<Path>>(
    service_path: T,
    target_path: T,
) -> anyhow::Result<(Service, Target)> {
    Ok((
        get_service_from_toml(service_path)?,
        get_target_from_toml(target_path)?,
    ))
}

pub fn get_service_from_toml<T: AsRef<Path>>(service_path: T) -> anyhow::Result<Service> {
    info!(
        "Getting service details {}",
        service_path.as_ref().display()
    );
    let file_content = read_to_string(service_path.as_ref())?;
    let service: Service = toml::from_str(&file_content).context(format!(
        "Failed to parse toml for service file {}",
        service_path.as_ref().display()
    ))?;
    Ok(service)
}

pub fn get_target_from_toml<T: AsRef<Path>>(target_path: T) -> anyhow::Result<Target> {
    info!("Getting target details {}", target_path.as_ref().display());
    let file_content = read_to_string(target_path.as_ref())?;
    let target: Target = toml::from_str(&file_content).context(format!(
        "Failed to parse toml for target file {}",
        target_path.as_ref().display()
    ))?;
    Ok(target)
}
