use crate::models::service::Service;

pub enum ShellCommand<'a> {
    CheckIfServiceExists(&'a Service),
}
