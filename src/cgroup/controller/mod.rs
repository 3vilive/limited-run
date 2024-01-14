use std::cell::Cell;

mod cgroup_v1;
mod cgroup_v2;

use crate::cgroup::SystemCgroupInfo;
use crate::common;
use anyhow::{Context, Ok, Result};
use cgroup_v1::CgroupV1Controller;
use cgroup_v2::CgroupV2Controller;

pub trait CgroupController {
    fn set_pid(&self, pid: u32);
    fn clean(&self) -> Result<()>;
    fn set_cpus(&self, cpus: f64) -> Result<()>;
    fn set_memory(&self, memory: String) -> Result<()>;
}

pub fn new_cgroup_controller(info: SystemCgroupInfo) -> Box<dyn CgroupController> {
    match info.cgroup_root_ver {
        1 => Box::new(CgroupV1Controller::with_sys_cgroup_info(info)),
        2 => Box::new(CgroupV2Controller::with_sys_cgroup_info(info)),
        _ => panic!("unexpected cgroup root ver"),
    }
}
