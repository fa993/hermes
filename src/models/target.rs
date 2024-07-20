use std::path::PathBuf;

use anyhow::{anyhow, Ok};
use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use log::info;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::os::{linux::Os, shell_cmd::ShellCommand, OsLike};

use super::{outcome::Outcome, service::Service};

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    pub name: String,
    address: Url,
    identity: PathBuf,
}

impl Target {
    async fn connect_to_server(&self) -> anyhow::Result<Client> {
        //TODO waiting on https://github.com/Miyoshi-Ryota/async-ssh2-tokio/issues/65
        let auth_method =
            AuthMethod::with_key_file(self.identity.as_path().to_string_lossy().as_ref(), None);
        let client = Client::connect(
            (
                self.address
                    .host_str()
                    .ok_or(anyhow!("No host in {}", self.address))?,
                22,
            ),
            self.address.username(),
            auth_method,
            ServerCheckMethod::NoCheck,
        )
        .await?;

        Ok(client)
    }
}

impl Target {
    pub async fn install(&self, service: &Service) -> anyhow::Result<Outcome> {
        // first ssh into machine and then execute commands
        info!("Connecting to server at {}", self.address);
        let client = self.connect_to_server().await?;
        info!("Connection established");

        let out = client
            .execute(Os::transpile(ShellCommand::CheckIfServiceExists, service)?.as_str())
            .await?;

        if out.exit_status != 0 {
            // service does not exist .. ssh the starter file and the starter script or install it
            info!("{} service does not exist", service.name())
        } else {
            info!("{} service already seems to be installed", service.name())
        }

        // do a git clone, git push

        info!("Starting service {}", service.name());
        let out = client
            .execute(Os::transpile(ShellCommand::EnableAndStartService, service)?.as_str())
            .await?;

        if out.exit_status == 0 {
            info!("Successfully started and enabled service");
            return Ok(Outcome::Success);
        } else {
            return Err(anyhow!("Couldn't Up service").context(out.stdout));
        }
    }
}
