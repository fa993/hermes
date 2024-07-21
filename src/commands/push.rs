use std::path::Path;

use log::info;

use crate::commands::utils::parse_args;

pub async fn push<T: AsRef<Path>>(service_path: T, target_path: T) -> anyhow::Result<()> {
    let (service, target) = parse_args(service_path, target_path)?;
    info!(
        "Pushing service {} to target {}",
        service.name(),
        target.name
    );
    target.push(&service).await
}
