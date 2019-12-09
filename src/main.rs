use chaperone::initialize;
use chaperone::ExecutionMode;
use chaperone::ExecutionMode::{Initialize, NoOp};
use chaperone::Filesystem;
use std::path::PathBuf;
use std::result::Result;

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn main() -> Result<(), std::io::Error> {
    let home = std::env::var("HOME").expect("Unable to locate home directory.");
    let mut filesystem = Filesystem::new(PathBuf::from(home));

    match execution_mode() {
        Initialize => initialize(&mut std::io::stdout(), &mut filesystem),
        NoOp => Ok(()),
    }
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
                .required(false),
        )
        .get_matches();

    if matches.is_present("init") {
        return ExecutionMode::Initialize;
    } else {
        return NoOp;
    }
}
