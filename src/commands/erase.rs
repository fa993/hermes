use std::path::Path;

use log::info;

use crate::{commands::utils::parse_args, models::remote::Remote};

pub async fn erase<T: AsRef<Path>>(service_path: T, target_path: T) -> anyhow::Result<()> {
    let (service, target) = parse_args(service_path, target_path)?;
    info!(
        "Erasing service {} in target {}",
        service.name(),
        target.name()
    );

    let remote = Remote::with(target).await?;

    remote.erase(&service).await
}
