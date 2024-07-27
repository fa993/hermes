use std::path::Path;

use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use log::info;
use rand::{distributions::Alphanumeric, Rng};

use super::{service::Service, target::Target};
use crate::anyhow;

#[derive(Debug)]
pub struct Connector {
    client: Client,
}

impl Connector {
    pub async fn new(target: &Target) -> anyhow::Result<Connector> {
        info!("Extracting socket address from {}", target.address());
        let addr = target
            .address()
            .socket_addrs(|| Some(22))
            .map_err(|f| anyhow!("Coudln't convert to socket addr").context(f))?;
        if addr.len() != 1 {
            return Err(anyhow!("Only single target is supported").context(
                addr.into_iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        let target_address = addr.first().unwrap();
        info!("Connecting to server at {}", target_address);
        //TODO waiting on https://github.com/Miyoshi-Ryota/async-ssh2-tokio/issues/65
        let auth_method =
            AuthMethod::with_key_file(target.identity().as_path().to_string_lossy().as_ref(), None);
        let client = Client::connect(
            *target_address,
            target.address().username(),
            auth_method,
            ServerCheckMethod::NoCheck,
        )
        .await?;

        info!("Connection established");

        Ok(Connector { client })
    }

    pub async fn exec(&self, command: &str) -> anyhow::Result<()> {
        let out = self.client.execute(command).await?;
        if out.exit_status != 0 {
            Err(anyhow!("Couldn't execute remote command").context(out.stdout))
        } else {
            Ok(())
        }
    }

    pub async fn transfer<T: AsRef<Path>>(&self, file: T) -> anyhow::Result<String> {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        self.client
            .upload_file(
                file.as_ref().to_string_lossy().into_owned().as_str(),
                s.as_str(),
            )
            .await?;

        Ok(s)
    }

    pub async fn transfer_env<T: AsRef<Path>>(
        &self,
        file: T,
        service: &Service,
    ) -> anyhow::Result<()> {
        self.client
            .upload_file(
                file.as_ref().to_string_lossy().into_owned().as_str(),
                format!("{}/.env", service.name()).as_str(),
            )
            .await?;
        Ok(())
    }

    pub async fn transfer_nginx<T: AsRef<Path>>(
        &self,
        file: T,
        service: &Service,
    ) -> anyhow::Result<()> {
        self.client
            .upload_file(
                file.as_ref().to_string_lossy().into_owned().as_str(),
                format!("/etc/nginx/default.d/{}.conf", service.name()).as_str(),
            )
            .await?;
        Ok(())
    }

    pub async fn transfer_starter<T: AsRef<Path>>(
        &self,
        file: T,
        service: &Service,
    ) -> anyhow::Result<()> {
        self.client
            .upload_file(
                file.as_ref().to_string_lossy().into_owned().as_str(),
                format!("{}-starter.sh", service.name()).as_str(),
            )
            .await?;
        Ok(())
    }

    pub async fn transfer_sysd<T: AsRef<Path>>(
        &self,
        file: T,
        service: &Service,
    ) -> anyhow::Result<()> {
        self.client
            .upload_file(
                file.as_ref().to_string_lossy().into_owned().as_str(),
                format!("/etc/systemd/system/{}.service", service.name()).as_str(),
            )
            .await?;
        Ok(())
    }
}
