use crate::ChaperoneError;
use crate::EnvironmentVariables;
use crate::FilesystemAccess;
use crate::Settings;
use std::io::Write;

pub fn command(
    _stdout: &mut Write,
    _filesystem: &mut dyn FilesystemAccess,
    settings: &mut Box<Settings>,
) -> Result<(), ChaperoneError> {
    let command = settings.command.as_mut();

    command.env(EnvironmentVariables::AccessKey.as_str(), "a");
    command.env(EnvironmentVariables::SecretKey.as_str(), "b");
    command.env(EnvironmentVariables::SessionToken.as_str(), "c");

    if let Ok(child) = command.spawn() {
        child
            .wait_with_output()
            .expect("Failure to execute command.");
    } else {
        return Err(ChaperoneError::CommandNotFound(
            settings.command_name.to_string(),
        ));
    }

    Ok(())
}
