pub mod args;
pub mod cgroup;
pub mod common;

use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use args::CliArgs;
use cgroup::CgroupController;
use log::{debug, error};

pub fn run(args: args::CliArgs) -> Result<()> {
    let mut app = App::new(args)?;
    app.run()?;
    app.quit()?;
    Ok(())
}

struct App {
    args: CliArgs,
    cgroup_controller: Box<dyn CgroupController>,
    cmd: Option<process::Child>,
}

impl App {
    pub fn new(args: CliArgs) -> Result<Self> {
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
            return Err(anyhow!("system cgroup does not support control cpus"));
        }

        Ok(Self {
            args,
            cgroup_controller: cgroup::new_cgroup_controller(system_cgroup_info),
            cmd: None,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // execute command
        let cmd = process::Command::new(&self.args.commands[0])
            .args(&self.args.commands[1..])
            .spawn()
            .with_context(|| "failed to spawn command")?;

        debug!("pid of cmd: {}", cmd.id());
        self.cgroup_controller.set_pid(cmd.id());

        if let Some(cpus) = self.args.cpus {
            self.cgroup_controller.set_cpus(cpus)?;
        }

        if let Some(ref mem) = self.args.memory {
            self.cgroup_controller.set_memory(mem.clone())?;
        }

        self.cmd = Some(cmd);

        self.wait()
    }

    pub fn wait(&mut self) -> Result<()> {
        // handle SIGTERM
        let term = Arc::new(AtomicBool::new(false));
        let term2 = term.clone();
        ctrlc::set_handler(move || term2.store(true, Ordering::Relaxed))
            .expect("Error setting Ctrl-C handler");

        while !term.load(Ordering::Relaxed) {
            // no term sign
            if let Some(ref mut cmd) = self.cmd {
                if cmd.try_wait()?.is_some() {
                    break;
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }

        if term.load(Ordering::Relaxed) {
            // received term
            if let Some(ref mut cmd) = self.cmd {
                if let Err(e) = cmd.kill() {
                    error!("failed to kill cmd: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn quit(&self) -> Result<()> {
        self.cgroup_controller
            .clean()
            .with_context(|| "failed to clean controller")?;

        Ok(())
    }
}
