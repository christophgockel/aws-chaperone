pub use error::ChaperoneError;
use fs::Filesystem;
use fs::FilesystemAccess;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::result::Result;
use ExecutionMode::{Initialize, Run};

mod config;
mod error;
mod fs;
mod init;
mod run;
mod settings;

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};
pub const CONFIGURATION_DIRECTORY_NAME: &str = ".chaperone";
pub const CONFIGURATION_FILE_NAME: &str = "config";
pub const CONFIGURATION_FILE_CONTENT: &str = "[example]
mfa-device-arn = arn:aws:iam::1234567890:mfa/user.name
aws-cli-profile = profile-name
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
    Run(Arguments),
}

pub struct Arguments {
    pub profile: String,
    pub command_parts: Vec<String>,
}

pub struct Settings {
    pub command_name: String,
    pub command: Box<Command>,
}

fn main() -> Result<(), Error> {
    let home = std::env::var("HOME").expect("Unable to locate home directory.");
    let mut filesystem = Filesystem::new(PathBuf::from(home));

    let execution_result = match execution_mode() {
        Ok(Initialize) => init::initialize(&mut std::io::stdout(), &mut filesystem),
        Ok(Run(mut arguments)) => {
            run_thing(&mut std::io::stdout(), &mut filesystem, &mut arguments)
        }
        Err(e) => Err(e),
    };

    if let Err(e) = execution_result {
        eprintln!("{}", e);

        process::exit(1);
    }

    Ok(())
}

fn run_thing(
    stdout: &mut Write,
    filesystem: &mut FilesystemAccess,
    arguments: &mut Arguments,
) -> Result<(), ChaperoneError> {
    let config_file_content = filesystem.read_config_file().unwrap();
    let c = config::Config::for_profile(arguments.profile.clone(), config_file_content)?;
    let mut settings = settings::build_settings(arguments, c);

    if let Err(e) = run::command(stdout, filesystem, &mut settings) {
        eprintln!("{}", e);

        process::exit(1);
    }

    Ok(())
}

fn execution_mode() -> Result<ExecutionMode, ChaperoneError> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("init")
                .help("Creates the initial configuration file(s)")
                .short("i")
                .long("init")
                .required(false)
                .conflicts_with("command"),
        )
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("command")
                .help("The command to execute with the temporary STS credentials,\ne.g. chaperone aws s3 ls ...")
                .required(true)
                .multiple(true)
                .conflicts_with("init"),
        )
        .get_matches();

    if matches.is_present("init") {
        return Ok(ExecutionMode::Initialize);
    } else {
        let profile =
            std::env::var("CHAPERONE_PROFILE").map_err(|_| ChaperoneError::MissingProfile)?;
        let command_arguments: Vec<&str> = matches.values_of("command").unwrap().collect();

        let arguments = Arguments {
            profile: profile,
            command_parts: command_arguments.iter().map(|p| p.to_string()).collect(),
        };

        return Ok(Run(arguments));
    }
}
