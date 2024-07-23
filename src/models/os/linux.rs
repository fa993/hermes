use std::path::Path;

use crate::models::service::Service;

use super::{shell_cmd::ShellCommand, OsLike};

const NO_OP: &str = ":";

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
            (ShellCommand::StopService, true) => Ok(NO_OP.to_string()),
            (ShellCommand::StopService, false) => {
                Ok(format!("sudo systemctl stop {}.service ", service.name(),))
            }
            (ShellCommand::EraseService, true) => Ok(NO_OP.to_string()),
            (ShellCommand::EraseService, false) => Ok(format!(
                "rm -rf /etc/systemd/system/{}.service && rm -rf {} && rm -rf {}-startup.sh",
                service.name(),
                service.name(),
                service.name()
            )),
        }
    }

    fn exec_script_once<T: AsRef<Path>>(script_path: T) -> anyhow::Result<String> {
        return Ok(format!(
            "chmod +x {} && ./{} && rm -rf {}",
            script_path.as_ref().display(),
            script_path.as_ref().display(),
            script_path.as_ref().display()
        ));
    }
}
