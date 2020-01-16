use crate::config::Config;
use crate::Arguments;
use crate::Settings;
use std::process::Command;

pub fn build_settings(arguments: &mut Arguments, _c: Config) -> Box<Settings> {
    let (command_name, command_arguments) = arguments.command_parts.split_first().unwrap();
    let mut command = Command::new(command_name);
    command.args(command_arguments);

    return Box::new(Settings {
        command_name: command_name.clone(),
        command: Box::new(command),
    });
}
