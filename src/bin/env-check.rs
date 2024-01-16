use anyhow::Result;

fn main() -> Result<()> {
    let system_cgroup_info = limited_run::cgroup::get_system_cgroup_info()?;
    println!("system cgroup info: {:#?}", system_cgroup_info);
    Ok(())
}
