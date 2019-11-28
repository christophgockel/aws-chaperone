use chaperone::initialize;
use chaperone::ExecutionMode;
use chaperone::ExecutionMode::{Initialize, NoOp};

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn main() {
    match execution_mode() {
        Initialize => initialize(&mut std::io::stdout()),
        NoOp => return,
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
