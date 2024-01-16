mod cgroup_v1;
mod cgroup_v2;

use crate::cgroup::SystemCgroupInfo;

use anyhow::Result;
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
        1 => Box::new(CgroupV1Controller::new()),
        2 => Box::new(CgroupV2Controller::new()),
        _ => panic!("unexpected cgroup root ver"),
    }
}
