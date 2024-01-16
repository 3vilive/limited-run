use std::{cell::Cell, fs, thread, time::Duration};

use anyhow::{Context, Result};

use crate::common;

use super::CgroupController;

#[derive(Default)]
pub struct CgroupV1Controller {
    pid: Cell<u32>,
    controlled_cpu: Cell<bool>,
    controlled_memory: Cell<bool>,
}

impl CgroupV1Controller {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get_cpu_control_dir(&self) -> String {
        // work dir: /sys/fs/cgroup/cpu/limited-run/{pid}
        common::get_cgroup_v1_control_dir(common::CGroupV1SubSystem::CPU, self.pid.get())
    }

    fn get_memory_control_dir(&self) -> String {
        // work dir: /sys/fs/cgroup/memory/limited-run/{pid}
        common::get_cgroup_v1_control_dir(common::CGroupV1SubSystem::Memory, self.pid.get())
    }

    fn append_pid_to_procs(&self, control_dir: &str) -> Result<()> {
        fs::write(
            format!("{}/cgroup.procs", control_dir),
            self.pid.get().to_string(),
        )
        .with_context(|| "failed to write cgroup.procs")
    }

    fn wait_procs_be_empty(&self, control_dir: &str) -> Result<()> {
        let procs_path = format!("{}/cgroup.procs", control_dir);
        loop {
            let content = fs::read_to_string(&procs_path)
                .with_context(|| format!("failed to read {}", procs_path))?;
            if content.is_empty() {
                break;
            } else {
                log::debug!("content: {}", content);
                thread::sleep(Duration::from_millis(1));
            }
        }

        Ok(())
    }

    fn remove_cpus_controller(&self) -> Result<()> {
        if !self.controlled_cpu.get() {
            return Ok(());
        }

        let control_dir = self.get_cpu_control_dir();

        // wait procs be empty
        self.wait_procs_be_empty(&control_dir)?;

        // remove cgroup dir
        fs::remove_dir(&control_dir)
            .with_context(|| format!("failed to remove {}", control_dir))?;

        Ok(())
    }

    fn remove_memory_controller(&self) -> Result<()> {
        if !self.controlled_memory.get() {
            return Ok(());
        }

        let control_dir = self.get_memory_control_dir();

        // wait procs be empty
        self.wait_procs_be_empty(&control_dir)?;

        // remove cgroup dir
        fs::remove_dir(&control_dir)
            .with_context(|| format!("failed to remove {}", control_dir))?;

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
        .with_context(|| "failed to set cpu.cfs_quota_us")?;

        // set tasks
        self.append_pid_to_procs(&control_dir)?;

        Ok(())
    }

    fn set_memory(&self, memory: String) -> Result<()> {
        self.controlled_memory.set(true);

        let control_dir: String = self.get_memory_control_dir();

        fs::create_dir_all(&control_dir)
            .with_context(|| format!("failed to create cgroup directory: {}", control_dir))?;

        // set memory.limit_in_bytes
        fs::write(format!("{}/memory.limit_in_bytes", control_dir), memory)
            .with_context(|| "failed to set memory.limit_in_bytes")?;

        // set tasks
        self.append_pid_to_procs(&control_dir)?;

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        self.remove_cpus_controller()?;
        self.remove_memory_controller()?;

        Ok(())
    }
}
