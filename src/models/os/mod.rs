use shell_cmd::ShellCommand;

use super::service::Service;

#[cfg(feature = "linux")]
pub mod linux;
pub mod shell_cmd;

pub trait OsLike {
    fn transpile(cmd: ShellCommand, service: &Service) -> anyhow::Result<String>;
}
