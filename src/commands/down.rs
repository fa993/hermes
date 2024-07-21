use std::path::Path;

use log::info;

use crate::commands::utils::parse_args;

pub async fn down<T: AsRef<Path>>(service_path: T, target_path: T) -> anyhow::Result<()> {
    let (service, target) = parse_args(service_path, target_path)?;
    info!(
        "Pulling service {} in target {} down",
        service.name(),
        target.name()
    );
    target.down(&service).await
}
