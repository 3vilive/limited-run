use anyhow::Result;
use limited_run::args;

fn main() -> Result<()> {
    env_logger::init();

    let args = args::parse_cli_args();
    log::debug!("args: {:#?}", args);

    limited_run::run(args)
}
