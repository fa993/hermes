use shell_cmd::ShellCommand;

#[cfg(feature = "linux")]
pub mod linux;
pub mod shell_cmd;

pub trait OsLike {
    fn transpile(cmd: ShellCommand) -> anyhow::Result<String>;
}
