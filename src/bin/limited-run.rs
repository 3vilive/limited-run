use anyhow::Result;
use limited_run::args;

fn main() -> Result<()> {
    let args = args::parse_cli_args();
    log::debug!("args: {:#?}", args);

    env_logger::init();

    limited_run::run(args)
}
