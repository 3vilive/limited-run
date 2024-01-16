use std::{
    cell::{Cell, RefCell},
    fs, thread,
    time::Duration,
};

use anyhow::{Context, Result};

use super::CgroupController;

pub struct CgroupV2Controller {
    pid: Cell<u32>,
    control_dir: RefCell<String>,
}

impl CgroupV2Controller {
    pub fn new() -> Self {
        Self {
            pid: Cell::new(0),
            control_dir: RefCell::new(String::new()),
        }
    }

    fn init_control_dir(&self) -> Result<()> {
        let control_dir = format!(
            "/sys/fs/cgroup/system.slice/limited-run-{}.scope",
            self.pid.get()
        );

        // create control dir
        fs::create_dir_all(&control_dir)
            .with_context(|| format!("failed to create cgroup directory: {}", control_dir))?;

        self.control_dir.replace(control_dir);

        // write cgroup.procs
        fs::write(
            format!("{}/cgroup.procs", self.control_dir.borrow()),
            self.pid.get().to_string(),
        )
        .with_context(|| "failed to write cgroup.procs")?;

        Ok(())
    }

    fn wait_procs_be_empty(&self) -> Result<()> {
        let procs_path = format!("{}/cgroup.procs", self.control_dir.borrow());
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
}

impl CgroupController for CgroupV2Controller {
    fn set_pid(&self, pid: u32) {
        self.pid.set(pid);
    }

    fn set_cpus(&self, cpus: f64) -> Result<()> {
        self.init_control_dir()?;

        // set cpu.max
        let cpu_max_path = format!("{}/cpu.max", self.control_dir.borrow());
        let content = format!("{} 100000", (cpus * 100000.0) as u64);
        fs::write(cpu_max_path, content).with_context(|| "failed to write cpu.max")?;

        Ok(())
    }

    fn set_memory(&self, memory: String) -> Result<()> {
        self.init_control_dir()?;

        // set memory.max
        let memory_max_path = format!("{}/memory.max", self.control_dir.borrow());
        fs::write(memory_max_path, memory).with_context(|| "failed to write memory.max")?;

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let control_dir = self.control_dir.borrow();
        if control_dir.is_empty() {
            return Ok(());
        }

        self.wait_procs_be_empty()?;

        fs::remove_dir(control_dir.as_str())
            .with_context(|| format!("failed to remove {}", control_dir))?;

        Ok(())
    }
}
