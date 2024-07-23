#[non_exhaustive]
pub enum ShellCommand {
    CheckIfServiceExists,
    EnableAndStartService,
    StopService,
    EraseService,
    CreateServiceFolder,
    GitInitService,
    CleanupInstallService,
}
