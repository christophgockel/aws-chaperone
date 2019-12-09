use std::fs;
use std::fs::create_dir;
use std::io;
use std::io::Error;
use std::path::PathBuf;

const CONFIGURATION_DIRECTORY_NAME: &str = ".chaperone";
const CONFIGURATION_FILE_NAME: &str = "config";
const CONFIGURATION_FILE_CONTENT: &str = "[example]
serial-number = arn:aws:iam::1234567890:mfa/user.name
aws-profile = profile-name
";

pub enum ExecutionMode {
    NoOp,
    Initialize,
}

pub fn initialize(
    stdout: &mut io::Write,
    filesystem: &mut dyn FilesystemAccess,
) -> Result<(), Error> {
    if !filesystem.config_directory_exists() {
        filesystem.create_config_directory()?;
    }

    if !filesystem.config_file_exists() {
        stdout.write("Creating configuration file.\n".as_bytes())?;
        filesystem.create_config_file(CONFIGURATION_FILE_CONTENT)?;
    } else {
        stdout.write("Configuration file already exists.\nNothing to do here.\n".as_bytes())?;
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    struct StdoutDouble {
        pub written_content: Option<String>,
    }

    impl StdoutDouble {
        fn new() -> StdoutDouble {
            StdoutDouble {
                written_content: None,
            }
        }
    }

    impl io::Write for StdoutDouble {
        fn write(&mut self, content: &[u8]) -> std::result::Result<usize, std::io::Error> {
            self.written_content = Some(std::str::from_utf8(content).unwrap().to_string());

            Ok(0)
        }
        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct FilesystemDouble {
        pub have_config_directory_exist: bool,
        pub have_config_file_exist: bool,

        pub create_config_directory_has_been_called: bool,
        pub create_config_file_has_been_called: bool,

        pub written_config_file_content: Option<String>,
    }

    impl FilesystemDouble {
        pub fn without_config_directory() -> FilesystemDouble {
            FilesystemDouble {
                have_config_directory_exist: false,
                have_config_file_exist: false,

                create_config_directory_has_been_called: false,
                create_config_file_has_been_called: false,

                written_config_file_content: None,
            }
        }

        pub fn with_config_directory() -> FilesystemDouble {
            FilesystemDouble {
                have_config_directory_exist: true,
                have_config_file_exist: false,

                create_config_directory_has_been_called: false,
                create_config_file_has_been_called: false,

                written_config_file_content: None,
            }
        }

        pub fn without_config_file() -> FilesystemDouble {
            FilesystemDouble {
                have_config_directory_exist: true,
                have_config_file_exist: false,

                create_config_directory_has_been_called: false,
                create_config_file_has_been_called: false,

                written_config_file_content: None,
            }
        }

        pub fn with_config_file() -> FilesystemDouble {
            FilesystemDouble {
                have_config_directory_exist: true,
                have_config_file_exist: true,

                create_config_directory_has_been_called: false,
                create_config_file_has_been_called: false,

                written_config_file_content: None,
            }
        }
    }

    impl FilesystemAccess for FilesystemDouble {
        fn config_directory_exists(&self) -> bool {
            self.have_config_directory_exist
        }

        fn config_file_exists(&self) -> bool {
            self.have_config_file_exist
        }

        fn create_config_directory(&mut self) -> Result<(), Error> {
            self.create_config_directory_has_been_called = true;

            Ok(())
        }

        fn create_config_file(&mut self, content: &str) -> Result<(), Error> {
            self.create_config_file_has_been_called = true;
            self.written_config_file_content = Some(content.to_string());

            Ok(())
        }
    }

    mod configuration_directory {
        use super::*;

        #[test]
        fn creates_config_directory_if_it_doesnt_exist_already() {
            let mut stdout = StdoutDouble::new();
            let mut filesystem = FilesystemDouble::without_config_directory();

            initialize(&mut stdout, &mut filesystem).unwrap();

            assert!(filesystem.create_config_directory_has_been_called);
        }

        #[test]
        fn does_not_create_config_directory_if_it_already_exists() {
            let mut stdout = StdoutDouble::new();
            let mut filesystem = FilesystemDouble::with_config_directory();

            initialize(&mut stdout, &mut filesystem).unwrap();

            assert!(!filesystem.create_config_directory_has_been_called);
        }

    }

    mod configuration_file {
        use super::*;

        mod does_not_exist {
            use super::*;

            #[test]
            fn creates_config_file_if_it_doesnt_exist_already() {
                let mut stdout = StdoutDouble::new();
                let mut filesystem = FilesystemDouble::without_config_file();

                initialize(&mut stdout, &mut filesystem).unwrap();

                assert!(filesystem.create_config_file_has_been_called);
                assert_eq!(
                    CONFIGURATION_FILE_CONTENT,
                    filesystem.written_config_file_content.unwrap()
                );
            }

            #[test]
            fn prints_message_when_creating_configuration_directory() {
                let mut stdout = StdoutDouble::new();
                let mut filesystem = FilesystemDouble::without_config_file();

                initialize(&mut stdout, &mut filesystem).unwrap();

                assert_eq!(
                    "Creating configuration file.\n",
                    stdout.written_content.unwrap()
                )
            }
        }

        mod exists {
            use super::*;

            #[test]
            fn does_not_create_config_file_if_it_does_already_exist() {
                let mut stdout = StdoutDouble::new();
                let mut filesystem = FilesystemDouble::with_config_file();

                initialize(&mut stdout, &mut filesystem).unwrap();

                assert!(!filesystem.create_config_file_has_been_called);
            }

            #[test]
            fn prints_nothing_to_do_here_message() {
                let mut stdout = StdoutDouble::new();
                let mut filesystem = FilesystemDouble::with_config_file();

                initialize(&mut stdout, &mut filesystem).unwrap();

                assert_eq!(
                    "Configuration file already exists.\nNothing to do here.\n",
                    stdout.written_content.unwrap()
                )
            }
        }
    }
}
