use chaperone::EnvironmentVariables;
use chaperone::FilesystemAccess;
use chaperone::Settings;
use std::io::{Error, ErrorKind, Write};

pub fn command(
    _stdout: &mut Write,
    _filesystem: &mut dyn FilesystemAccess,
    settings: &mut Box<Settings>,
) -> Result<(), Error> {
    let command = settings.command.as_mut();

    command.env(EnvironmentVariables::AccessKey.as_str(), "a");
    command.env(EnvironmentVariables::SecretKey.as_str(), "b");
    command.env(EnvironmentVariables::SessionToken.as_str(), "c");

    if let Ok(child) = command.spawn() {
        child.wait_with_output()?;
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Command not found: {}", settings.command_name),
        ));
    }

    Ok(())
}
