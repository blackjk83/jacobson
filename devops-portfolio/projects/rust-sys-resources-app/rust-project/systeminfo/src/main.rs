use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use systeminfo::{get_system_info, SystemInfoError};
use clap::Parser;
use chrono::Local;
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Watch mode: continuously update the display
    #[arg(short, long)]
    watch: bool,

    /// Update interval in seconds for watch mode
    #[arg(short = 'n', long, default_value_t = 1)]
    interval: u64,

    /// Show detailed view instead of summary
    #[arg(short, long)]
    detailed: bool,
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:>6.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:>6.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:>6.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:>6.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{:>6} B ", bytes)  // Extra space to align with other units
    }
}

fn colorize_percentage(value: f32, is_storage: bool) -> String {
    let formatted = format!("{:>5.1}%", value);
    if is_storage {
        // Memory and disk thresholds (85/60)
        if value >= 85.0 {
            formatted.red().to_string()
        } else if value >= 60.0 {
            formatted.yellow().to_string()
        } else {
            formatted.green().to_string()
        }
    } else {
        // CPU thresholds (80/50)
        if value >= 80.0 {
            formatted.red().to_string()
        } else if value >= 50.0 {
            formatted.yellow().to_string()
        } else {
            formatted.green().to_string()
        }
    }
}

fn colorize_temperature(temp: f32) -> String {
    let formatted = format!("{:>5.1}°C", temp);
    if temp >= 75.0 {
        formatted.red().to_string()
    } else if temp >= 55.0 {
        formatted.yellow().to_string()
    } else {
        formatted.green().to_string()
    }
}

fn create_usage_bar(percentage: f32, width: usize, is_storage: bool) -> String {
    let filled_chars = ((percentage / 100.0) * width as f32) as usize;
    let empty_chars = width - filled_chars;
    let bar = format!(
        "[{}{}]",
        "=".repeat(filled_chars),
        " ".repeat(empty_chars)
    );
    
    // Use different thresholds for storage (memory/disk) vs CPU
    if is_storage {
        if percentage >= 85.0 {
            bar.red().to_string()
        } else if percentage >= 60.0 {
            bar.yellow().to_string()
        } else {
            bar.green().to_string()
        }
    } else {
        if percentage >= 80.0 {
            bar.red().to_string()
        } else if percentage >= 50.0 {
            bar.yellow().to_string()
        } else {
            bar.green().to_string()
        }
    }
}

fn get_display_height(info: &systeminfo::SystemInfo, detailed: bool) -> usize {
    let mut lines = 1; // Single-line header
    
    if detailed {
        lines += 2; // CPU header + global
        lines += info.cpu_cores.len(); // CPU cores
        lines += 4; // Memory section (1 header + 2 content + 1 spacing)
        lines += 2; // Disk header + spacing
        lines += info.disks.len(); // Disks
        lines += 2; // Network header + spacing
        lines += info.networks.len(); // Networks
    } else {
        lines += 2; // CPU + Memory
        lines += info.disks.len(); // Disks
        lines += info.networks.len(); // Networks
    }
    
    lines
}

fn display_system_info(is_first_run: bool, previous_height: Option<usize>, detailed: bool) -> Result<usize, SystemInfoError> {
    let info = get_system_info()?;
    let display_height = get_display_height(&info, detailed);
    
    if !is_first_run {
        // Move cursor up and clear to end of screen
        if let Some(prev_height) = previous_height {
            print!("\x1B[{}A\x1B[J", prev_height);
        }
    } else {
        // On first run, clear screen and move to top
        print!("\x1B[2J\x1B[H");
    }
    
    // Single-line compact header with timestamp
    println!("{}│{}│Ctrl+C to exit", 
        "System Monitor".cyan(),
        Local::now().format(" %H:%M:%S ").to_string().cyan());

    if detailed {
        // CPU Section
        println!("CPU Usage/Temp");
        println!("Global {} {} {}",
            create_usage_bar(info.global_cpu.usage, 30, false),
            colorize_percentage(info.global_cpu.usage, false),
            info.global_cpu.temperature.map_or("".into(), |t| format!("[{}]", colorize_temperature(t)))
        );

        for (i, cpu) in info.cpu_cores.iter().enumerate() {
            print!("Core{:<2} {} {}", 
                i,
                create_usage_bar(cpu.usage, 30, false),
                colorize_percentage(cpu.usage, false)
            );
            if let Some(temp) = cpu.temperature {
                print!(" [{}]", colorize_temperature(temp));
            }
            println!();
        }

        // Memory Section
        let ram_usage = (info.memory.used as f32 / info.memory.total as f32 * 100.0) as f32;
        println!("\nMemory");
        println!("RAM  {} {} {:>13} / {:<13}",
            create_usage_bar(ram_usage, 30, true),
            colorize_percentage(ram_usage, true),
            format_bytes(info.memory.used),
            format_bytes(info.memory.total)
        );
        
        let swap_usage = (info.memory.swap_used as f32 / info.memory.swap_total as f32 * 100.0) as f32;
        println!("Swap {} {} {:>13} / {:<13}",
            create_usage_bar(swap_usage, 30, true),
            colorize_percentage(swap_usage, true),
            format_bytes(info.memory.swap_used),
            format_bytes(info.memory.swap_total)
        );

        // Disk Section
        println!("\nDisk Usage");
        for disk in &info.disks {
            let usage = (disk.used as f64 / disk.total as f64 * 100.0) as f32;
            println!("{:<12} {} {} {:>13} / {:<13}",
                format!("{}:", disk.mount_point),
                create_usage_bar(usage, 20, true),
                colorize_percentage(usage, true),
                format_bytes(disk.used),
                format_bytes(disk.total)
            );
        }

        // Network Section
        println!("\nNetwork");
        for net in &info.networks {
            println!("{:<12} {}", 
                format!("{}:", net.interface),
                net.ip_address.cyan()
            );
        }
    } else {
        // Summary View
        println!("CPU    {} {}",
            create_usage_bar(info.global_cpu.usage, 40, false),
            colorize_percentage(info.global_cpu.usage, false)
        );

        let ram_usage = (info.memory.used as f32 / info.memory.total as f32 * 100.0) as f32;
        println!("Memory {} {} {:>13} / {:<13}",
            create_usage_bar(ram_usage, 40, true),
            colorize_percentage(ram_usage, true),
            format_bytes(info.memory.used),
            format_bytes(info.memory.total)
        );

        for disk in &info.disks {
            let usage = (disk.used as f64 / disk.total as f64 * 100.0) as f32;
            println!("{:<8} {} {} {:>13} / {:<13}",
                format!("{}:", disk.mount_point),
                create_usage_bar(usage, 30, true),
                colorize_percentage(usage, true),
                format_bytes(disk.used),
                format_bytes(disk.total)
            );
        }

        for net in &info.networks {
            println!("{:<8} {}", 
                format!("{}:", net.interface),
                net.ip_address.cyan()
            );
        }
    }

    io::stdout().flush()?;
    Ok(display_height)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    if cli.watch {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            print!("\r\x1B[K");
            println!("\nMonitoring stopped.");
            io::stdout().flush().ok();
        })?;

        // Clear screen and move to top
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;
        
        let mut is_first_run = true;
        let mut previous_height = None;
        while running.load(Ordering::SeqCst) {
            match display_system_info(is_first_run, previous_height, cli.detailed) {
                Ok(height) => {
                    previous_height = Some(height);
                    is_first_run = false;
                }
                Err(e) => {
                    eprintln!("{}", format!("Error: {}", e).red());
                    return Err(e.into());
                }
            }
            thread::sleep(Duration::from_secs(cli.interval));
        }
    } else {
        if let Err(e) = display_system_info(true, None, cli.detailed) {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }

    Ok(())
}
