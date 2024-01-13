use std::{fs, process};

use crate::cgroup::SystemCgroupInfo;
use crate::common;
use anyhow::{Context, Result};

pub trait CgroupController {
    fn initialize(&mut self) -> Result<()>;
    fn set_cpus(&self, cpus: f64) -> Result<()>;
}

pub fn new_with_cgroup_info(info: SystemCgroupInfo) -> Box<dyn CgroupController> {
    if info.cgroup_root_ver == 1 {
        Box::new(CgroupV1Controller::with_sys_cgroup_info(info))
    } else {
        Box::new(CgroupV1Controller::with_sys_cgroup_info(info))
    }
}

pub struct CgroupV1Controller {
    initialized: bool,
    pid: u32,
    sys_cgroup_info: SystemCgroupInfo,
}

impl CgroupV1Controller {
    pub fn with_sys_cgroup_info(info: SystemCgroupInfo) -> Self {
        Self {
            initialized: false,
            pid: 0,
            sys_cgroup_info: info,
        }
    }
}

impl CgroupController for CgroupV1Controller {
    fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        // get own pid
        self.pid = get_pid()?;
        self.initialized = true;

        Ok(())
    }

    fn set_cpus(&self, cpus: f64) -> Result<()> {
        // work dir: /sys/fs/cgroup/cpu/limited-run/{pid}
        let control_dir: String = format!("{}/limited-run/{}", common::CGROUP_V1_CPU_DIR, self.pid);
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
            self.pid.to_string(),
        )
        .with_context(|| "failed to write cgroup.procs")?;
        Ok(())
    }
}

pub fn get_pid() -> Result<u32> {
    let pid = process::id();
    Ok(pid)
}

pub struct CgroupV2Controller {
    initialized: bool,
    pid: u32,
    sys_cgroup_info: SystemCgroupInfo,
}

impl CgroupV2Controller {
    pub fn with_sys_cgroup_info(info: SystemCgroupInfo) -> Self {
        Self {
            initialized: false,
            pid: 0,
            sys_cgroup_info: info,
        }
    }
}

impl CgroupController for CgroupV2Controller {
    fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        self.pid = get_pid().with_context(|| "failed to get pid")?;

        self.initialized = true;
        Ok(())
    }

    fn set_cpus(&self, cpus: f64) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}
