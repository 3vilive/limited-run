use crate::common;
use anyhow::Result;
use std::{fs, process};

#[derive(Debug, Default)]
pub struct SystemCgroupInfo {
    pub support_v1: bool,
    pub support_v2: bool,
    pub cgroup_root_ver: u8,
    pub support_control_cpu: bool,
}

pub fn get_system_cgroup_info() -> Result<SystemCgroupInfo> {
    let mut info: SystemCgroupInfo = SystemCgroupInfo::default();

    // check cgroup version
    (info.support_v1, info.support_v2) = get_cgroup_version_supports()?;

    // check cgroup root directory version
    if info.support_v1 || info.support_v2 {
        info.cgroup_root_ver = get_cgroup_root_ver()?;
    }

    info.support_control_cpu = is_support_control_cpu(info.cgroup_root_ver)?;

    Ok(info)
}

fn get_cgroup_version_supports() -> Result<(bool, bool)> {
    // check cgroup version
    let output = process::Command::new("sh")
        .arg("-c")
        .arg("grep cgroup /proc/filesystems")
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "check cgroup version failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let mut support_v1 = false;
    let mut support_v2 = false;
    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.lines().for_each(|line| {
        if line.ends_with("cgroup") {
            support_v1 = true;
        } else if line.ends_with("cgroup2") {
            support_v2 = true;
        }
    });

    Ok((support_v1, support_v2))
}

fn get_cgroup_root_ver() -> Result<u8> {
    let is_v2_cgroup_root = fs::read_dir(common::CGROUP_ROOT_DIR)?
        .map(|x| x.map(|y| y.file_name()))
        .map(|x| x.unwrap_or_default())
        .any(|x| x == "cgroup.controllers");

    Ok(if is_v2_cgroup_root { 2 } else { 1 })
}

fn is_support_control_cpu(cgroup_ver: u8) -> Result<bool> {
    match cgroup_ver {
        1 => {
            let exists_cfs_quota_us = fs::read_dir(common::CGROUP_V1_CPU_DIR)?
                .map(|x| x.map(|y| y.file_name()))
                .map(|x| x.unwrap_or_default())
                .any(|x| x == "cpu.cfs_quota_us");

            Ok(exists_cfs_quota_us)
        }
        2 => Ok(true),
        _ => Ok(false),
    }
}
