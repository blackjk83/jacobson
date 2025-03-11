use sysinfo::{System, SystemExt, CpuExt, ComponentExt, DiskExt, NetworkExt};
use thiserror::Error;
use std::io;
use std::process::Command;

#[derive(Error, Debug)]
pub enum SystemInfoError {
    #[error("Failed to get CPU information")]
    CpuError,
    #[error("Failed to get memory information")]
    MemoryError,
    #[error("Failed to get temperature information")]
    TemperatureError,
    #[error("Failed to get disk information")]
    DiskError,
    #[error("Failed to get network information")]
    NetworkError,
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug)]
pub struct CpuInfo {
    pub usage: f32,
    pub idle: f32,
    pub temperature: Option<f32>,
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_free: u64,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

#[derive(Debug)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip_address: String,
}

#[derive(Debug)]
pub struct SystemInfo {
    pub global_cpu: CpuInfo,
    pub cpu_cores: Vec<CpuInfo>,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,  // Changed to Vec to show all active interfaces
}

fn get_interface_ip(interface: &str) -> Option<String> {
    // Use ip addr show to get interface information
    let output = Command::new("ip")
        .args(["addr", "show", interface])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Look for inet (IPv4) addresses
    for line in output_str.lines() {
        let line = line.trim();
        if line.starts_with("inet ") && !line.contains("scope host") {
            // Extract IP address (format: inet 192.168.1.2/24 ...)
            if let Some(ip) = line.split_whitespace()
                .nth(1)  // Get the IP/mask
                .and_then(|s| s.split('/').next())  // Remove the mask
            {
                // Don't return loopback addresses
                if !ip.starts_with("127.") {
                    return Some(ip.to_string());
                }
            }
        }
    }
    None
}

fn is_valid_interface(interface: &str) -> bool {
    // Filter out virtual and container interfaces
    !interface.starts_with("lo") && 
    !interface.starts_with("veth") && 
    !interface.starts_with("flannel") &&
    !interface.starts_with("docker") &&
    !interface.starts_with("br-") &&
    !interface.starts_with("vxlan") &&
    !interface.starts_with("cni") &&
    !interface.starts_with("tun") &&
    !interface.starts_with("tap") &&
    !interface.starts_with("virbr")
}

pub fn get_system_info() -> Result<SystemInfo, SystemInfoError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get CPU information
    let mut cpu_cores = Vec::new();
    let global_usage = sys.global_cpu_info().cpu_usage();
    let global_idle = 100.0 - global_usage;
    
    for cpu in sys.cpus() {
        cpu_cores.push(CpuInfo {
            usage: cpu.cpu_usage(),
            idle: 100.0 - cpu.cpu_usage(),
            temperature: None, // Will be updated below
        });
    }

    // Get temperatures and match them to CPUs
    for component in sys.components() {
        let label = component.label().to_lowercase();
        if label.contains("core") {
            if let Some(core_num) = label.split_whitespace().last() {
                if let Ok(index) = core_num.parse::<usize>() {
                    if index < cpu_cores.len() {
                        cpu_cores[index].temperature = Some(component.temperature());
                    }
                }
            }
        }
    }

    // Get memory information
    let memory = MemoryInfo {
        total: sys.total_memory() * 1024,  // Convert from KB to bytes
        used: sys.used_memory() * 1024,
        free: sys.free_memory() * 1024,
        available: sys.available_memory() * 1024,
        swap_total: sys.total_swap() * 1024,
        swap_used: sys.used_swap() * 1024,
        swap_free: (sys.total_swap() - sys.used_swap()) * 1024,
    };

    // Get disk information
    let mut disks = Vec::new();
    for disk in sys.disks() {
        disks.push(DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            total: disk.total_space(),
            used: disk.total_space() - disk.available_space(),
            free: disk.available_space(),
        });
    }

    // Get network information
    let mut networks = Vec::new();
    for (interface_name, data) in sys.networks() {
        // Only consider physical interfaces with traffic
        if is_valid_interface(&interface_name) && data.total_received() > 0 {
            if let Some(ip) = get_interface_ip(&interface_name) {
                networks.push(NetworkInfo {
                    interface: interface_name.to_string(),
                    ip_address: ip,
                });
            }
        }
    }

    // If no active interfaces were found, show N/A
    if networks.is_empty() {
        networks.push(NetworkInfo {
            interface: "N/A".to_string(),
            ip_address: "N/A".to_string(),
        });
    }

    Ok(SystemInfo {
        global_cpu: CpuInfo {
            usage: global_usage,
            idle: global_idle,
            temperature: sys.components().iter()
                .find(|c| c.label().to_lowercase().contains("package"))
                .map(|c| c.temperature()),
        },
        cpu_cores,
        memory,
        disks,
        networks,
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
        assert!(info.global_cpu.usage >= 0.0, "CPU usage should be non-negative");
        assert!(info.memory.used <= info.memory.total, "Used memory should not exceed total");
    }
}
