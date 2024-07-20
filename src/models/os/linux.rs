use crate::models::service::ServiceKind;
use anyhow::anyhow;

use super::{shell_cmd::ShellCommand, OsLike};

pub struct Os;

impl OsLike for Os {
    fn transpile(cmd: ShellCommand) -> anyhow::Result<String> {
        match cmd {
            ShellCommand::CheckIfServiceExists(service)
                if *service.kind() != ServiceKind::External =>
            {
                Ok(format!(
                    "[ -e /etc/systemd/system/{}.service ]",
                    service.name()
                ))
            }
            ShellCommand::CheckIfServiceExists(service)
                if *service.kind() == ServiceKind::External =>
            {
                Ok(format!("command -v {}", service.name()))
            }
            _ => Err(anyhow!("Command type not supported")),
        }
    }
}
