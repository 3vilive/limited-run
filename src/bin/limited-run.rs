use std::process;

use anyhow::Result;
use limited_run::args;

fn main() -> Result<()> {
    env_logger::init();

    let args = args::parse_cli_args();
    log::debug!("args: {:#?}", args);

    if sudo::check() != sudo::RunningAs::Root {
        log::error!("The program requires root user privileges to run.");
        process::exit(-1);
    }

    limited_run::run(args)
}
