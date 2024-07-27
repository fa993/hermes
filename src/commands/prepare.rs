use std::path::Path;

use anyhow::anyhow;
use log::info;
use tokio::process::Command;

use crate::commands::utils::get_service_from_toml;

pub async fn prepare<T: AsRef<Path>>(
    service_path: T,
    proxy_command: Vec<String>,
) -> anyhow::Result<()> {
    let service = get_service_from_toml(service_path)?;
    info!("Preparing service {}", service.name());

    let mut proxy = proxy_command.iter();

    let cmd = proxy
        .next()
        .ok_or(anyhow!("No command found").context(proxy_command.join(" ")))?;

    let mut os_cmd = Command::new(cmd);
    if let Some(port) = service.source().get_port() {
        os_cmd.env("PORT", port.to_string());
    }
    os_cmd
        .env("SUBPATH", format!("/{}/", service.name()))
        .args(proxy)
        .spawn()?
        .wait()
        .await?;
    Ok(())
}
