pub struct CgroupPaths {
    pub root_dir: &'static str,
    pub v1_cpu_dir: &'static str,
}

pub const CGROUP_ROOT_DIR: &'static str = "/sys/fs/cgroup";
pub const CGROUP_V1_CPU_DIR: &'static str = "/sys/fs/cgroup/cpu";
pub const CGROUP_V1_MEMORY_DIR: &'static str = "/sys/fs/cgroup/memory";

#[derive(Debug)]
pub enum CGroupV1SubSystem {
    CPU,
    Memory,
}

impl CGroupV1SubSystem {
    pub fn get_dir(self) -> &'static str {
        match self {
            Self::CPU => CGROUP_V1_CPU_DIR,
            Self::Memory => CGROUP_V1_MEMORY_DIR,
        }
    }
}

pub fn get_cgroup_v1_control_dir(sub_system: CGroupV1SubSystem, pid: u32) -> String {
    format!("{}/limited-run/{}", sub_system.get_dir(), pid)
}

pub fn get_cgroup_v1_tasks_path(sub_system: CGroupV1SubSystem, pid: u32) -> String {
    format!("{}/limited-run/{}/tasks", sub_system.get_dir(), pid)
}
