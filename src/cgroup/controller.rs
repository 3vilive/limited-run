use std::{cell::Cell, fs, process};

use crate::cgroup::SystemCgroupInfo;
use crate::common;
use anyhow::{Context, Ok, Result};

pub trait CgroupController {
    fn set_pid(&self, pid: u32);
    fn clean(&self) -> Result<()>;
    fn set_cpus(&self, cpus: f64) -> Result<()>;
}

pub fn new_cgroup_controller(info: SystemCgroupInfo) -> Box<dyn CgroupController> {
    match info.cgroup_root_ver {
        1 => Box::new(CgroupV1Controller::with_sys_cgroup_info(info)),
        2 => Box::new(CgroupV2Controller::with_sys_cgroup_info(info)),
        _ => panic!("unexpected cgroup root ver"),
    }
}

#[derive(Default)]
pub struct CgroupV1Controller {
    sys_cgroup_info: SystemCgroupInfo,
    pid: Cell<u32>,
    controlled_cpu: Cell<bool>,
}

impl CgroupV1Controller {
    pub fn with_sys_cgroup_info(info: SystemCgroupInfo) -> Self {
        Self {
            sys_cgroup_info: info,
            ..Default::default()
        }
    }

    fn get_cpu_control_dir(&self) -> String {
        // work dir: /sys/fs/cgroup/cpu/limited-run/{pid}
        format!(
            "{}/limited-run/{}",
            common::CGROUP_V1_CPU_DIR,
            self.pid.get()
        )
    }

    fn clean_cpus(&self) -> Result<()> {
        let cpu_control_dir = self.get_cpu_control_dir();

        // remove cgroup.procs
        fs::write(format!("{}/cgroup.procs", cpu_control_dir), "")
            .with_context(|| "failed to remove cgroup.proces")?;

        // remove cpu control dir
        fs::remove_dir(cpu_control_dir).with_context(|| "failed to remove cpu control dir")?;

        Ok(())
    }
}

impl CgroupController for CgroupV1Controller {
    fn set_pid(&self, pid: u32) {
        self.pid.set(pid);
    }

    fn set_cpus(&self, cpus: f64) -> Result<()> {
        self.controlled_cpu.set(true);

        let control_dir: String = self.get_cpu_control_dir();
        fs::create_dir_all(&control_dir)
            .with_context(|| format!("failed to create cgroup directory: {}", control_dir))?;

        // set cfs quota
        let cfs_quota_us = (cpus * 100000.0) as u64;
        fs::write(
            format!("{}/cpu.cfs_quota_us", control_dir),
            cfs_quota_us.to_string(),
        )
        .with_context(|| "failed to write cpu.cfs_quota_us")?;

        // set cgroup.procs
        fs::write(
            format!("{}/cgroup.procs", control_dir),
            self.pid.get().to_string(),
        )
        .with_context(|| "failed to write cgroup.procs")?;

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        if self.controlled_cpu.get() {
            // clean cpu
            self.clean_cpus()?;
        }

        Ok(())
    }
}

pub struct CgroupV2Controller {
    initialized: bool,
    pid: Cell<u32>,
    sys_cgroup_info: SystemCgroupInfo,
}

impl CgroupV2Controller {
    pub fn with_sys_cgroup_info(info: SystemCgroupInfo) -> Self {
        Self {
            initialized: false,
            pid: Cell::new(0),
            sys_cgroup_info: info,
        }
    }
}

impl CgroupController for CgroupV2Controller {
    fn set_pid(&self, pid: u32) {
        self.pid.set(pid);
    }

    fn set_cpus(&self, cpus: f64) -> Result<()> {
        unimplemented!();
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        unimplemented!();
    }
}
