use std::process;

use crate::cgroup::SystemCgroupInfo;
use anyhow::Result;

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
        CgroupV1Controller {
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
        // work dir: /cgroup/cpu/limited-run/{pid}
        // let cfs_quota_us = (cpus * 100000) as u64;
        // fs::write(format!("{}/cpu.cfs_quota_us", common::CGROUP_V1_CPU_DIR), cfs_quota_us.to_string())?;
        Ok(())
    }
}

pub fn get_pid() -> Result<u32> {
    let pid = process::id();
    Ok(pid)
}
