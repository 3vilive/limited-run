use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Number of CPUs (examples: 0.5,1)
    #[arg(long, value_parser = cpus_parser)]
    pub cpus: Option<f64>,

    /// Memory limit（examples: 10k,10m,10g）
    #[arg(long, value_parser = memory_parser)]
    pub memory: Option<String>,

    // commands to run
    pub commands: Vec<String>,
}

pub fn parse_cli_args() -> CliArgs {
    let args = CliArgs::parse();

    // check commands
    if args.commands.is_empty() {
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

fn memory_parser(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("invalid memory setting".to_string());
    }

    // check value
    let value_s = &s[..s.len() - 1];
    if !value_s.chars().all(|c| c.is_ascii_digit()) {
        return Err("invalid memory setting".to_string());
    }

    // check unit
    let unit = match s.chars().last().take() {
        None => return Err("invalid memory setting".to_string()),
        Some(unit) => unit,
    };

    if unit.is_ascii_digit() {
        return Err("missing unit".to_string());
    }

    match unit {
        'k' | 'K' | 'm' | 'M' | 'g' | 'G' => (),
        _ => return Err("invalid memory unit".to_string()),
    };

    Ok(s.to_string())
}
