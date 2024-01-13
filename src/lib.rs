pub mod args;
pub mod cgroup;
pub mod common;

use std::process;

use anyhow::Result;
use log::debug;

pub fn run() -> Result<()> {
    env_logger::init();

    let args = args::parse_cli_args();
    log::debug!("args: {:#?}", args);

    let system_cgroup_info = cgroup::get_system_cgroup_info()?;
    log::debug!(
        "cgroup v1: {}, cgroup v2: {}",
        system_cgroup_info.support_v1,
        system_cgroup_info.support_v2
    );
    log::debug!(
        "cgroup root version: {}",
        system_cgroup_info.cgroup_root_ver,
    );

    // check if system support control cpus
    if args.cpus.is_some() && !system_cgroup_info.support_control_cpu {
        log::error!("system cgroup does not support cpus");
        return Ok(());
    }

    // create cgroup controller
    let mut cgroup_controller = cgroup::new_with_cgroup_info(system_cgroup_info);

    // init controller
    cgroup_controller.initialize()?;

    if let Some(cpus) = args.cpus {
        cgroup_controller.set_cpus(cpus)?;
    }

    let pid = cgroup::get_pid()?;
    debug!("limited-run pid: {}", pid);

    // execute command
    let mut cmd = process::Command::new(&args.commands[0]);
    cmd.args(&args.commands[1..])
        .status()
        .expect("failed to execute command");

    Ok(())
}
