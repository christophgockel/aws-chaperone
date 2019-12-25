use chaperone::Arguments;
use chaperone::ExecutionMode;
use chaperone::ExecutionMode::{Initialize, Run};
use chaperone::Filesystem;
use chaperone::FilesystemAccess;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process;
use std::result::Result;

mod config;
mod init;
mod run;
mod settings;

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};

fn main() -> Result<(), std::io::Error> {
    let home = std::env::var("HOME").expect("Unable to locate home directory.");
    let mut filesystem = Filesystem::new(PathBuf::from(home));

    match execution_mode() {
        Initialize => init::initialize(&mut std::io::stdout(), &mut filesystem),
        Run(mut arguments) => run_thing(&mut std::io::stdout(), &mut filesystem, &mut arguments),
    }
}

fn run_thing(
    stdout: &mut Write,
    filesystem: &mut FilesystemAccess,
    arguments: &mut Arguments,
) -> Result<(), Error> {
    let config_file_content = filesystem.read_config_file().unwrap();
    let c = match config::Config::for_profile(arguments.profile.clone(), config_file_content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);

            process::exit(1);
        }
    };

    let mut settings = settings::build_settings(arguments, c);

    if let Err(e) = run::command(stdout, filesystem, &mut settings) {
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
        let profile = std::env::var("CHAPERONE_PROFILE").expect("No CHAPERONE_PROFILE defined.");
        let command_arguments: Vec<&str> = matches.values_of("command").unwrap().collect();

        let arguments = Arguments {
            profile: profile,
            command_parts: command_arguments.iter().map(|p| p.to_string()).collect(),
        };

        return Run(arguments);
    }
}
