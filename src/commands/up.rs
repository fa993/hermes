use log::info;
use std::path::Path;

use crate::commands::utils::parse_args;

pub async fn up<T: AsRef<Path>>(service_path: T, target_path: T) -> anyhow::Result<()> {
    let (service, target) = parse_args(service_path, target_path)?;
    info!(
        "Deploying service {} to target {}",
        service.name(),
        target.name
    );
    service.install(&target).await
}
