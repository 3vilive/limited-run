use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Number of CPUs (0.5)
    #[arg(long, value_parser = cpus_parser)]
    pub cpus: Option<f64>,

    // commands to run
    pub commands: Vec<String>,
}

pub fn parse_cli_args() -> CliArgs {
    let args = CliArgs::parse();

    // check commands
    if args.commands.len() == 0 {
        let mut cmd = CliArgs::command();
        cmd.error(ErrorKind::MissingRequiredArgument, "commands is required")
            .exit()
    }

    args
}

fn cpus_parser(s: &str) -> Result<f64, String> {
    let num = s
        .parse::<f64>()
        .map_err(|_| format!("{} is not a valid decimal", s))?;

    if num <= 0.01 {
        Err("must be greater than 0.01".to_string())
    } else {
        Ok(num)
    }
}
