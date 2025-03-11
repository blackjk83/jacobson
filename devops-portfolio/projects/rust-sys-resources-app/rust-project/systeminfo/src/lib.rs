use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt};
use thiserror::Error;
use std::time::Duration;
use std::process::Command;

#[derive(Error, Debug)]
pub enum SystemInfoError {
    #[error("Failed to collect system information")]
    CollectionError,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Service error: {0}")]
    ServiceError(String),
    #[error("Signal handler error: {0}")]
    SignalError(String),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemMetrics {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub disks: Vec<DiskMetrics>,
    pub networks: Vec<NetworkMetrics>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CpuMetrics {
    pub usage: f32,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiskMetrics {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NetworkMetrics {
    pub interface: String,
    pub ip_address: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub struct MetricsCollector {
    sys: System,
    pub update_interval: Duration,
}

impl MetricsCollector {
    pub fn new(update_interval: Duration) -> Self {
        Self {
            sys: System::new_all(),
            update_interval,
        }
    }

    pub async fn collect(&mut self) -> Result<SystemMetrics, SystemInfoError> {
        self.sys.refresh_all();
        
        Ok(SystemMetrics {
            cpu: self.collect_cpu_metrics()?,
            memory: self.collect_memory_metrics()?,
            disks: self.collect_disk_metrics()?,
            networks: self.collect_network_metrics()?,
        })
    }

    fn collect_cpu_metrics(&self) -> Result<CpuMetrics, SystemInfoError> {
        Ok(CpuMetrics {
            usage: self.sys.global_cpu_info().cpu_usage(),
            temperature: None,
        })
    }

    fn collect_memory_metrics(&self) -> Result<MemoryMetrics, SystemInfoError> {
        Ok(MemoryMetrics {
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),
            available: self.sys.available_memory(),
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
        })
    }

    fn collect_disk_metrics(&self) -> Result<Vec<DiskMetrics>, SystemInfoError> {
        Ok(self.sys.disks().iter().map(|disk| {
            DiskMetrics {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total: disk.total_space(),
                used: disk.total_space() - disk.available_space(),
                available: disk.available_space(),
            }
        }).collect())
    }

    fn collect_network_metrics(&self) -> Result<Vec<NetworkMetrics>, SystemInfoError> {
        let mut metrics = Vec::new();
        for (interface_name, data) in self.sys.networks() {
            if let Some(ip) = get_interface_ip(interface_name) {
                metrics.push(NetworkMetrics {
                    interface: interface_name.clone(),
                    ip_address: ip,
                    rx_bytes: data.total_received(),
                    tx_bytes: data.total_transmitted(),
                });
            }
        }
        Ok(metrics)
    }
}

fn get_interface_ip(interface: &str) -> Option<String> {
    let output = Command::new("ip")
        .args(["addr", "show", interface])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    for line in output_str.lines() {
        if line.contains("inet ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return Some(parts[1].split('/').next().unwrap_or("").to_string());
            }
        }
    }

    None
}

pub mod service;
