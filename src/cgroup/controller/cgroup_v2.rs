use std::cell::Cell;

use crate::cgroup::SystemCgroupInfo;
use anyhow::{Context, Result};

use super::CgroupController;

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
    }

    fn set_memory(&self, memory: String) -> Result<()> {
        unimplemented!()
    }

    fn clean(&self) -> Result<()> {
        unimplemented!();
    }
}
