use crate::CONFIGURATION_DIRECTORY_NAME;
use crate::CONFIGURATION_FILE_NAME;
use std::convert::From;
use std::fs;
use std::fs::create_dir;
use std::io::Error;
use std::path::PathBuf;

pub trait FilesystemAccess {
    fn config_directory_exists(&self) -> bool;
    fn config_file_exists(&self) -> bool;
    fn create_config_directory(&mut self) -> Result<(), Error>;
    fn create_config_file(&mut self, content: &str) -> Result<(), Error>;
    fn read_config_file(&self) -> Result<String, Error>;
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

    fn read_config_file(&self) -> Result<String, Error> {
        fs::read_to_string(&self.path_to_configuration_file)
    }
}
