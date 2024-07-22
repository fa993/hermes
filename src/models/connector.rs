use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use log::info;

use super::target::Target;
use crate::anyhow;

#[derive(Debug)]
pub struct Connector {
    client: Client,
}

impl Connector {
    pub async fn new(target: &Target) -> anyhow::Result<Connector> {
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
}
