use crate::models::service::Service;
use anyhow::anyhow;

use super::{shell_cmd::ShellCommand, OsLike};

const NO_OP: &'static str = ":";

pub struct Os;

impl OsLike for Os {
    fn transpile(cmd: ShellCommand, service: &Service) -> anyhow::Result<String> {
        match (cmd, service.source().is_tool()) {
            (ShellCommand::CheckIfServiceExists, false) => Ok(format!(
                "[ -e /etc/systemd/system/{}.service ]",
                service.name()
            )),
            (ShellCommand::CheckIfServiceExists, true) => {
                Ok(format!("command -v {}", service.name()))
            }
            (ShellCommand::EnableAndStartService, true) => Ok(NO_OP.to_string()),
            (ShellCommand::EnableAndStartService, false) => Ok(format!(
                "sudo systemctl enable {}.service && sudo systemctl restart {}.service",
                service.name(),
                service.name()
            )),
            #[allow(unreachable_patterns)]
            (_, _) => Err(anyhow!("Command type not supported")),
        }
    }
}
