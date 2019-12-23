use chaperone::ExecutionMode;
use chaperone::ExecutionMode::{Initialize, Run};
use chaperone::Filesystem;
use chaperone::FilesystemAccess;
use chaperone::Settings;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::result::Result;

mod config;
mod init;
mod run;

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};

fn main() -> Result<(), std::io::Error> {
    let home = std::env::var("HOME").expect("Unable to locate home directory.");
    let _profile = std::env::var("CHAPERONE_PROFILE").expect("No CHAPERONE_PROFILE defined.");
    let mut filesystem = Filesystem::new(PathBuf::from(home));

    match execution_mode() {
        Initialize => init::initialize(&mut std::io::stdout(), &mut filesystem),
        Run(mut config) => run_thing(&mut std::io::stdout(), &mut filesystem, &mut config),
    }
}

fn run_thing(
    stdout: &mut Write,
    filesystem: &mut dyn FilesystemAccess,
    settings: &mut Settings,
) -> Result<(), Error> {
    let config_file_content = filesystem.read_config_file().unwrap();
    let _c = config::Config::for_profile("dev".to_string(), config_file_content);

    if let Err(e) = run::command(stdout, filesystem, settings) {
        eprintln!("{}", e);

        process::exit(1);
    }

    Ok(())
}

fn execution_mode() -> ExecutionMode {
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
        return ExecutionMode::Initialize;
    } else {
        let command_parts: Vec<&str> = matches.values_of("command").unwrap().collect();

        if let Some((first, rest)) = command_parts.split_first() {
            let mut command = Box::new(Command::new(first));
            command.args(rest);

            let settings = Settings {
                command_name: first.to_string(),
                command: command,
            };

            return Run(settings);
        }
    }

    return ExecutionMode::Initialize;
}
