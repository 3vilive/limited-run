pub struct CgroupPaths {
    pub root_dir: &'static str,
    pub v1_cpu_dir: &'static str,
}

pub const CGROUP_ROOT_DIR: &'static str = "/sys/fs/cgroup";
pub const CGROUP_V1_CPU_DIR: &'static str = "/sys/fs/cgroup/cpu";

pub const CGROUP_PATHS: CgroupPaths = CgroupPaths {
    root_dir: CGROUP_ROOT_DIR,
    v1_cpu_dir: CGROUP_V1_CPU_DIR,
};
