use sysinfo::{System, SystemExt, CpuExt, ComponentExt};
use thiserror::Error;
use std::io;

#[derive(Debug)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub components_temp: Vec<(String, f32)>,
}

#[derive(Error, Debug)]
pub enum SystemInfoError {
    #[error("Failed to retrieve system information")]
    SystemError,
    #[error("Memory usage exceeds total memory")]
    InvalidMemoryState,
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

/// Retrieves current system information including CPU, memory, and temperature
/// 
/// # Returns
/// - `Result<SystemInfo, SystemInfoError>`: System information or error
/// 
/// # Examples
/// ```
/// use systeminfo::get_system_info;
/// 
/// let sys_info = get_system_info().expect("Failed to get system info");
/// println!("CPU Usage: {}%", sys_info.cpu_usage);
/// println!("Memory Used: {} bytes", sys_info.memory_used);
/// ```
pub fn get_system_info() -> Result<SystemInfo, SystemInfoError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    
    // Validate memory usage
    if memory_used > memory_total {
        return Err(SystemInfoError::InvalidMemoryState);
    }
    
    let components_temp = sys.components()
        .iter()
        .map(|comp| (comp.label().to_string(), comp.temperature()))
        .collect();

    Ok(SystemInfo {
        cpu_usage,
        memory_used,
        memory_total,
        components_temp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_info() {
        let result = get_system_info();
        assert!(result.is_ok(), "Should successfully get system info");
        
        let info = result.unwrap();
        assert!(info.cpu_usage >= 0.0, "CPU usage should be non-negative");
        assert!(info.memory_used <= info.memory_total, "Used memory should not exceed total");
    }
}
