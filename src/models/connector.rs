use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use getset::Getters;
use log::info;

use super::target::Target;
use crate::anyhow;

#[derive(Debug, Default)]
pub struct Connector {
    client: Option<Client>,
}

impl Connector {
    pub async fn init(&mut self, target: &Target) -> anyhow::Result<()> {
        info!("Connecting to server at {}", target.address());
        //TODO waiting on https://github.com/Miyoshi-Ryota/async-ssh2-tokio/issues/65
        let auth_method =
            AuthMethod::with_key_file(target.identity().as_path().to_string_lossy().as_ref(), None);
        let client = Client::connect(
            (
                target
                    .address()
                    .host_str()
                    .ok_or(anyhow!("No host in {}", target.address()))?,
                22,
            ),
            target.address().username(),
            auth_method,
            ServerCheckMethod::NoCheck,
        )
        .await?;

        info!("Connection established");

        self.client = Some(client);
        Ok(())
    }

    pub async fn exec(&self, command: &str) -> anyhow::Result<ExecResult> {
        assert!(self.client.is_some());
        let client = self.client.as_ref().unwrap();
        let out = client.execute(command).await?;
        Ok(ExecResult {
            exit_code: out.exit_status,
            stdout: out.stdout,
        })
    }
}

#[derive(Getters, Debug)]
#[get = "pub"]
pub struct ExecResult {
    exit_code: u32,
    stdout: String,
}
