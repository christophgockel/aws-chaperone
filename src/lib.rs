use std::fs;
use std::fs::create_dir;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

pub const CONFIGURATION_DIRECTORY_NAME: &str = ".chaperone";
pub const CONFIGURATION_FILE_NAME: &str = "config";
pub const CONFIGURATION_FILE_CONTENT: &str = "[example]
serial-number = arn:aws:iam::1234567890:mfa/user.name
aws-profile = profile-name
";
pub const ENVIRONMENT_VARIABLE_FOR_ACCESS_KEY: &str = "AWS_ACCESS_KEY_ID";
pub const ENVIRONMENT_VARIABLE_FOR_SECRET_KEY: &str = "AWS_SECRET_ACCESS_KEY";
pub const ENVIRONMENT_VARIABLE_FOR_SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";

pub enum EnvironmentVariables {
    AccessKey,
    SecretKey,
    SessionToken,
}

impl EnvironmentVariables {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnvironmentVariables::AccessKey => "AWS_ACCESS_KEY_ID",
            EnvironmentVariables::SecretKey => "AWS_SECRET_ACCESS_KEY",
            EnvironmentVariables::SessionToken => "AWS_SESSION_TOKEN",
        }
    }
}

pub enum ExecutionMode {
    Initialize,
    Run(Configuration),
}

pub struct Configuration {
    pub command_name: String,
    pub command: Box<Command>,
}

pub trait FilesystemAccess {
    fn config_directory_exists(&self) -> bool;
    fn config_file_exists(&self) -> bool;
    fn create_config_directory(&mut self) -> Result<(), Error>;
    fn create_config_file(&mut self, content: &str) -> Result<(), Error>;
}

pub struct Filesystem {
    path_to_configuration_directory: PathBuf,
    path_to_configuration_file: PathBuf,
}

impl Filesystem {
    pub fn new(user_home_directory: PathBuf) -> Filesystem {
        let mut path_to_configuration_directory = PathBuf::from(user_home_directory);
        path_to_configuration_directory.push(CONFIGURATION_DIRECTORY_NAME);

        let mut path_to_configuration_file = PathBuf::from(&path_to_configuration_directory);
        path_to_configuration_file.push(CONFIGURATION_FILE_NAME);

        Filesystem {
            path_to_configuration_directory,
            path_to_configuration_file,
        }
    }
}

impl FilesystemAccess for Filesystem {
    fn config_directory_exists(&self) -> bool {
        self.path_to_configuration_directory.exists()
    }

    fn config_file_exists(&self) -> bool {
        self.path_to_configuration_file.exists()
    }

    fn create_config_directory(&mut self) -> Result<(), Error> {
        create_dir(self.path_to_configuration_directory.as_path())
    }

    fn create_config_file(&mut self, content: &str) -> Result<(), Error> {
        fs::write(self.path_to_configuration_file.as_path(), content)
    }
}
