use std::error::Error;
use systeminfo::{get_system_info, SystemInfoError};

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Main function that retrieves and displays system information
/// 
/// # Returns
/// - `Result<(), Box<dyn Error>>`: Result of the main function
/// 
/// # Examples
/// ```
/// use systeminfo::get_system_info;
/// 
/// let sys_info = get_system_info().expect("Failed to get system info");
/// println!("CPU Usage: {}%", sys_info.cpu_usage);
/// println!("Memory Used: {} bytes", sys_info.memory_used);
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    match get_system_info() {
        Ok(info) => {
            println!("System Resource Monitor");
            println!("=====================");
            println!("CPU Usage: {:.1}%", info.cpu_usage);
            println!("\nMemory:");
            println!("  Used:  {}", format_bytes(info.memory_used));
            println!("  Total: {}", format_bytes(info.memory_total));
            println!("  Usage: {:.1}%", (info.memory_used as f64 / info.memory_total as f64) * 100.0);
            
            if !info.components_temp.is_empty() {
                println!("\nTemperatures:");
                for (component, temp) in info.components_temp {
                    println!("  {}: {:.1}°C", component, temp);
                }
            }
            Ok(())
        }
        Err(SystemInfoError::SystemError) => {
            eprintln!("Error: Failed to retrieve system information");
            std::process::exit(1);
        }
        Err(SystemInfoError::InvalidMemoryState) => {
            eprintln!("Error: Invalid memory state detected");
            std::process::exit(1);
        }
    }
}
