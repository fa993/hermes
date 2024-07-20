use crate::models::service::{Service, ServiceKind};
use anyhow::anyhow;

use super::{shell_cmd::ShellCommand, OsLike};

const NO_OP: &'static str = ":";

pub struct Os;

impl OsLike for Os {
    fn transpile(cmd: ShellCommand, service: &Service) -> anyhow::Result<String> {
        match (cmd, service.kind()) {
            (ShellCommand::CheckIfServiceExists, ServiceKind::External) => Ok(format!(
                "[ -e /etc/systemd/system/{}.service ]",
                service.name()
            )),
            (ShellCommand::CheckIfServiceExists, _) => Ok(format!("command -v {}", service.name())),
            (ShellCommand::EnableAndStartService, ServiceKind::External) => Ok(NO_OP.to_string()),
            (ShellCommand::EnableAndStartService, _) => Ok(format!(
                "sudo systemctl enable {}.service && sudo systemctl restart {}.service",
                service.name(),
                service.name()
            )),
            #[allow(unreachable_patterns)]
            (_, _) => Err(anyhow!("Command type not supported")),
        }
    }
}
