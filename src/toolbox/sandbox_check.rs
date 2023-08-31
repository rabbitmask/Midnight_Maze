use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use std::time::{Duration, SystemTime};
use winapi::um::sysinfoapi::GetTickCount64;

pub(crate) fn get_system_memory() -> Option<(f64, f64)> {
    let mut memory_status: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

    if unsafe { GlobalMemoryStatusEx(&mut memory_status) } != 0 {
        let total_physical_memory = memory_status.ullTotalPhys as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_physical_memory = memory_status.ullAvailPhys as f64 / (1024.0 * 1024.0 * 1024.0);
        Some((total_physical_memory, available_physical_memory))
    } else {
        None
    }
}


fn get_system_memory_demo(){
    if let Some((total_memory, available_memory)) = get_system_memory() {
        if total_memory < 40 as f64 {
            println!("{}","ok");
        }
        println!("{:.0}", total_memory);
        println!("总物理内存：{:.2} GB", total_memory);
        println!("可用物理内存：{:.2} GB", available_memory);
    } else {
        println!("无法获取系统内存信息");
    }
}

fn get_system_uptime() -> Duration {
    let uptime_ms = unsafe { GetTickCount64() };
    Duration::from_millis(uptime_ms)
}

fn get_system_uptime_demo() {
    let system_uptime = get_system_uptime();
    let uptime_hours = system_uptime.as_secs() / 60;
    println!("Windows 运行持续时间：{} 分钟", uptime_hours);
}





pub(crate) fn sandbox_check_memory(min_memory:i32){
    if let Some((total_memory, available_memory)) = get_system_memory() {
        if total_memory < min_memory as f64 {
            std::process::exit(0);
        }
    } else {
        println!("无法获取系统内存信息");
    }
}


pub(crate) fn sandbox_check_uptime(min_uptime:i32){
    let system_uptime = get_system_uptime();
    let uptime_hours = system_uptime.as_secs() / 60;
    // println!("Windows 运行持续时间：{} 分钟", uptime_hours);
    if uptime_hours < min_uptime as u64 {
        std::process::exit(0);
    }
}

