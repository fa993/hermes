use log::info;
use std::path::Path;

use crate::{commands::utils::parse_args, models::remote::Remote};

pub async fn up<T: AsRef<Path>>(service_path: T, target_path: T) -> anyhow::Result<()> {
    let (service, target) = parse_args(service_path, target_path)?;
    info!(
        "Deploying service {} to target {}",
        service.name(),
        target.name(),
    );
    let remote = Remote::with(target).await?;

    remote.install_all(&service).await
}
