use std::collections::HashMap;

use super::{connector::Connector, service::Service, target::Target};

use anyhow::anyhow;
use log::info;
use tempfile::TempDir;
use tokio::process::Command;

use crate::{
    commands::utils::get_service_from_toml,
    models::{
        config_file::ConfigFileBuilder,
        os::{linux::Os, shell_cmd::ShellCommand, OsLike},
        service::SourceType,
    },
};

#[derive(Debug)]
pub struct Remote {
    target: Target,
    connector: Connector,
}

impl Remote {
    pub async fn with(target: Target) -> anyhow::Result<Remote> {
        let connector = Connector::new(&target).await?;
        Ok(Self { target, connector })
    }

    pub async fn push(&self, service: &Service) -> anyhow::Result<()> {
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
                format!("ssh -i {}", self.target.identity().as_path().display()),
            )
            .arg("push")
            .arg(self.target.address().as_str())
            .spawn()?
            .wait()
            .await?;

        if !out.success() {
            return Err(anyhow!("Couldn't git push"));
        }

        info!("Starting service {}", service.name());
        self.connector
            .exec(Os::transpile(ShellCommand::EnableAndStartService, service)?.as_str())
            .await?;

        info!("Successfully started and enabled service");
        Ok(())
    }

    pub async fn install_only(&self, service: &Service, is_dependency: bool) -> anyhow::Result<()> {
        // first ssh into machine and then execute commands
        let out = self
            .connector
            .exec(Os::transpile(ShellCommand::CheckIfServiceExists, service)?.as_str())
            .await;

        if out.is_err() {
            //TODO: service does not exist .. ssh the starter file and the starter script and git init bare or install it
            info!("{} service does not exist", service.name());
            match service.source() {
                SourceType::Git { env, port, .. } => {
                    let builder = ConfigFileBuilder::new()?;
                    let mut configs = HashMap::new();
                    configs.insert("name", service.name().clone());
                    configs.insert("port", port.to_string());
                    configs.insert("cmd", service.kind().get_startup_cmd().to_string());
                    configs.insert("username", self.target.address().username().to_string());
                    // generate starter file according to template
                    let starter_file = builder.create_starter(&configs)?;
                    // generate systemd file according to template
                    let sysd_file = builder.create_systemd(&configs)?;
                    // generate nginx file according to template
                    let nginx_file = builder.create_nginx(&configs)?;
                    // scp it along with dotenv
                    self.connector.transfer(env.as_path()).await?;
                    // mkdir folder with service name
                    // git init --bare it
                    todo!()
                }
                SourceType::Tool { install } => {
                    // ssh and run the install script
                    // scp install script
                    // run it
                    // delete it
                    info!("Transfering script to remote");
                    self.connector.transfer(install).await?;
                    info!("Executing setup script");
                    self.connector
                        .exec(Os::exec_script_once(install)?.as_str())
                        .await?;
                    info!("Exec script finished up");
                }
            };
        } else {
            info!("{} service already seems to be installed", service.name())
        }

        if !is_dependency {
            self.push(service).await
        } else {
            Ok(())
        }
    }

    pub async fn install_all(&self, service: &Service) -> anyhow::Result<()> {
        for dep in service.dependencies() {
            // read service file
            let service = get_service_from_toml(dep)?;
            // skip restart option
            self.install_only(&service, true).await?;
        }

        self.install_only(&service, false).await
    }

    pub async fn down(&self, service: &Service) -> anyhow::Result<()> {
        if service.source().is_tool() {
            return Err(anyhow!("Cannot pull a tool down"));
        }

        self.connector
            .exec(Os::transpile(ShellCommand::StopService, service)?.as_str())
            .await?;

        info!("{} service stopped", service.name());

        Ok(())
    }

    pub async fn erase(&self, service: &Service) -> anyhow::Result<()> {
        if service.source().is_tool() {
            return Err(anyhow!("Cannot erase a tool"));
        }

        self.connector
            .exec(Os::transpile(ShellCommand::StopService, service)?.as_str())
            .await?;

        info!("{} service stopped", service.name());

        // rm -rf the /etc/systemd/system/{service}.service
        // rm -rf the git src folder
        // rm -rf the startup file
        self.connector
            .exec(Os::transpile(ShellCommand::EraseService, service)?.as_str())
            .await?;

        info!("{} service erased", service.name());

        Ok(())
    }
}
