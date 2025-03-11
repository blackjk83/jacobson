use clap::Parser;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{self, Write};
use colored::*;
use systeminfo::{SystemInfoError, service};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run in watch mode with continuous updates
    #[arg(short, long)]
    watch: bool,

    /// Update interval in seconds (default: 1)
    #[arg(short = 'n', long, default_value = "1")]
    interval: u64,

    /// Run as a service with API endpoints
    #[arg(short, long)]
    service: bool,

    /// Port number for the service (default: 3000)
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), SystemInfoError> {
    let cli = Cli::parse();
    let update_interval = Duration::from_secs(cli.interval);

    if cli.service {
        println!("Starting system monitoring service on port {}", cli.port);
        println!("Available endpoints:");
        println!("  - GET /metrics           (Prometheus format)");
        println!("  - GET /api/v1/metrics    (JSON format)");
        println!("  - GET /health            (Health check)");
        
        service::run_service(cli.port, update_interval).await?;
    } else {
        if cli.watch {
            run_watch_mode(update_interval).await?;
        } else {
            run_single_update().await?;
        }
    }

    Ok(())
}

async fn run_watch_mode(interval: Duration) -> Result<(), SystemInfoError> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        print!("\r\x1B[K");
        println!("\nMonitoring stopped.");
        io::stdout().flush().unwrap_or(());
    }).map_err(|e| SystemInfoError::SignalError(e.to_string()))?;

    print!("\x1B[2J\x1B[H");
    io::stdout().flush()?;
    
    let mut previous_height = None;

    while running.load(Ordering::SeqCst) {
        match display_metrics().await {
            Ok(height) => {
                if let Some(prev_height) = previous_height {
                    for _ in 0..prev_height {
                        print!("\x1B[1A\x1B[2K");
                    }
                }
                previous_height = Some(height);
            }
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                return Err(e);
            }
        }
        tokio::time::sleep(interval).await;
    }

    Ok(())
}

async fn run_single_update() -> Result<(), SystemInfoError> {
    match display_metrics().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            Err(e)
        }
    }
}

async fn display_metrics() -> Result<usize, SystemInfoError> {
    let mut collector = systeminfo::MetricsCollector::new(Duration::from_secs(1));
    let metrics = collector.collect().await?;
    
    let mut output = Vec::new();

    // CPU Usage
    let cpu_status = match metrics.cpu.usage {
        u if u >= 80.0 => format!("{:.1}%", u).red(),
        u if u >= 50.0 => format!("{:.1}%", u).yellow(),
        u => format!("{:.1}%", u).green(),
    };
    output.push(format!("CPU Usage: {}", cpu_status));

    // CPU Temperature
    if let Some(temp) = metrics.cpu.temperature {
        let temp_status = match temp {
            t if t >= 75.0 => format!("{:.1}°C", t).red(),
            t if t >= 55.0 => format!("{:.1}°C", t).yellow(),
            t => format!("{:.1}°C", t).green(),
        };
        output.push(format!("CPU Temperature: {}", temp_status));
    }

    // Memory Usage
    let mem_percent = (metrics.memory.used as f32 / metrics.memory.total as f32) * 100.0;
    let mem_status = match mem_percent {
        m if m >= 85.0 => format!("{:.1}%", m).red(),
        m if m >= 60.0 => format!("{:.1}%", m).yellow(),
        m => format!("{:.1}%", m).green(),
    };
    output.push(format!("Memory Usage: {} ({} / {} GB)", 
        mem_status,
        metrics.memory.used / 1024 / 1024 / 1024,
        metrics.memory.total / 1024 / 1024 / 1024
    ));

    // Disk Usage
    for disk in metrics.disks {
        let used_percent = (disk.used as f32 / disk.total as f32) * 100.0;
        let disk_status = match used_percent {
            d if d >= 85.0 => format!("{:.1}%", d).red(),
            d if d >= 60.0 => format!("{:.1}%", d).yellow(),
            d => format!("{:.1}%", d).green(),
        };
        output.push(format!("Disk {} ({}): {} ({} / {} GB)",
            disk.name,
            disk.mount_point,
            disk_status,
            disk.used / 1024 / 1024 / 1024,
            disk.total / 1024 / 1024 / 1024
        ));
    }

    // Network Usage
    for net in metrics.networks {
        output.push(format!("Network {} ({}): ↓ {} MB/s ↑ {} MB/s",
            net.interface,
            net.ip_address,
            net.rx_bytes / 1024 / 1024,
            net.tx_bytes / 1024 / 1024
        ));
    }

    for line in output.iter() {
        println!("{}", line);
    }
    io::stdout().flush()?;

    Ok(output.len())
}
