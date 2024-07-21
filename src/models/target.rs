use std::path::PathBuf;

use anyhow::anyhow;
use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use log::info;
use serde::{Deserialize, Serialize};
use temp_dir::TempDir;
use tokio::process::Command;
use url::Url;

use crate::models::os::{linux::Os, shell_cmd::ShellCommand, OsLike};

use super::service::Service;

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

    async fn do_push(&self, client: Client, service: &Service) -> anyhow::Result<()> {
        if !service.source().is_git() {
            info!("Tools are not pushed");
            return Ok(());
        }

        let git_repo = service.source().get_repo_url().ok_or(anyhow!(
            "Incorrect Enum Variant for push {:?}",
            service.source()
        ))?;

        // git clone in tmp dir then git push to ssh,
        info!("Creating temporary directory");
        let d = TempDir::new()?;

        let out = Command::new("git")
            .arg("clone")
            .arg(git_repo.as_str())
            .arg(d.path())
            .spawn()?
            .wait()
            .await?;

        if !out.success() {
            return Err(anyhow!("Couldn't git clone"));
        }

        let out = Command::new("git")
            .env(
                "GIT_SSH_COMMAND",
                format!("ssh -i {}", self.identity.as_path().display()),
            )
            .arg("push")
            .arg(self.address.as_str())
            .spawn()?
            .wait()
            .await?;

        if !out.success() {
            return Err(anyhow!("Couldn't git push"));
        }

        info!("Starting service {}", service.name());
        let out = client
            .execute(Os::transpile(ShellCommand::EnableAndStartService, service)?.as_str())
            .await?;

        if out.exit_status == 0 {
            info!("Successfully started and enabled service");
            Ok(())
        } else {
            Err(anyhow!("Couldn't Start service").context(out.stdout))
        }
    }
}

impl Target {
    pub async fn push(&self, service: &Service) -> anyhow::Result<()> {
        info!("Connecting to server at {}", self.address);
        let client = self.connect_to_server().await?;
        info!("Connection established");
        self.do_push(client, service).await
    }

    pub async fn install(&self, service: &Service) -> anyhow::Result<()> {
        // first ssh into machine and then execute commands
        info!("Connecting to server at {}", self.address);
        let client = self.connect_to_server().await?;
        info!("Connection established");

        let out = client
            .execute(Os::transpile(ShellCommand::CheckIfServiceExists, service)?.as_str())
            .await?;

        if out.exit_status != 0 {
            //TODO: service does not exist .. ssh the starter file and the starter script and git init bare or install it
            info!("{} service does not exist", service.name())
        } else {
            info!("{} service already seems to be installed", service.name())
        }

        self.do_push(client, service).await
    }

    pub async fn down(&self, service: &Service) -> anyhow::Result<()> {
        if service.source().is_tool() {
            return Err(anyhow!("Cannot pull a tool down"));
        }

        info!("Connecting to server at {}", self.address);
        let client = self.connect_to_server().await?;
        info!("Connection established");

        let out = client
            .execute(Os::transpile(ShellCommand::StopService, service)?.as_str())
            .await?;

        if out.exit_status != 0 {
            Err(anyhow!("Couldn't stop service").context(out.stdout))
        } else {
            info!("{} service stopped", service.name());
            Ok(())
        }
    }

    pub async fn erase(&self, service: &Service) -> anyhow::Result<()> {
        if service.source().is_tool() {
            return Err(anyhow!("Cannot erase a tool"));
        }

        info!("Connecting to server at {}", self.address);
        let client = self.connect_to_server().await?;
        info!("Connection established");

        let out = client
            .execute(Os::transpile(ShellCommand::StopService, service)?.as_str())
            .await?;

        if out.exit_status != 0 {
            return Err(anyhow!("Couldn't stop service").context(out.stdout));
        };
        info!("{} service stopped", service.name());

        // rm -rf the /etc/systemd/system/{service}.service
        // rm -rf the git src folder
        // rm -rf the startup file
        let out = client
            .execute(Os::transpile(ShellCommand::EraseService, service)?.as_str())
            .await?;

        if out.exit_status != 0 {
            return Err(anyhow!("Couldn't erase service").context(out.stdout));
        };

        info!("{} service erased", service.name());

        Ok(())
    }
}
