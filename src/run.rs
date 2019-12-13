use chaperone::Configuration;
use chaperone::EnvironmentVariables;
use chaperone::FilesystemAccess;
use std::io::{Error, ErrorKind, Write};

pub fn command(
    _stdout: &mut Write,
    _filesystem: &mut dyn FilesystemAccess,
    config: &mut Configuration,
) -> Result<(), Error> {
    let command = config.command.as_mut();

    command.env(EnvironmentVariables::AccessKey.as_str(), "a");
    command.env(EnvironmentVariables::SecretKey.as_str(), "b");
    command.env(EnvironmentVariables::SessionToken.as_str(), "c");

    if let Ok(child) = command.spawn() {
        child.wait_with_output()?;
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Command not found: {}", config.command_name),
        ));
    }

    Ok(())
}
