use std::path::Path;

use shell_cmd::ShellCommand;

use super::service::Service;

#[cfg(feature = "linux")]
pub mod linux;
pub mod shell_cmd;

pub trait OsLike {
    fn transpile(cmd: ShellCommand, service: &Service) -> anyhow::Result<String>;

    fn exec_script<T: AsRef<Path>>(script_path: T) -> anyhow::Result<String>;
}
