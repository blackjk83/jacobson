use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use systeminfo::{get_system_info, SystemInfoError};
use clap::Parser;
use tabled::{Table, Tabled};
use tabled::settings::{Style, Modify, Alignment};
use tabled::settings::object::Segment;
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
}

#[derive(Tabled)]
struct CpuInfo {
    #[tabled(rename = "CPU Usage")]
    usage: String,
}

#[derive(Tabled)]
struct MemoryInfo {
    #[tabled(rename = "Memory")]
    category: String,
    value: String,
}

#[derive(Tabled)]
struct TempInfo {
    component: String,
    #[tabled(rename = "Temperature")]
    temp: String,
}

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

fn create_styled_table<T: Tabled>(data: T) -> String {
    Table::new([data])
        .with(Style::modern())
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .to_string()
}

fn create_styled_table_vec<T: Tabled>(data: Vec<T>) -> String {
    Table::new(data)
        .with(Style::modern())
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .to_string()
}

fn get_display_height(info: &systeminfo::SystemInfo) -> usize {
    // Calculate total lines needed for display
    let mut lines = 4; // Header + separator + timestamp + blank line
    lines += 4; // CPU table (header + separator + value + bottom)
    lines += 6; // Memory table (header + 2 separators + 3 values + bottom)
    if !info.components_temp.is_empty() {
        lines += 2 + info.components_temp.len() * 2; // Temperature table (header + components * 2 for separator)
    }
    lines
}

fn clear_previous_output(height: usize) {
    // Move cursor up
    print!("\x1B[{}A", height);
    // Clear from cursor to end of screen
    print!("\x1B[J");
}

fn display_system_info(is_first_run: bool, previous_height: Option<usize>) -> Result<usize, SystemInfoError> {
    let info = get_system_info()?;
    let display_height = get_display_height(&info);
    
    if !is_first_run {
        if let Some(prev_height) = previous_height {
            clear_previous_output(prev_height);
        }
    }
    
    // Header with timestamp
    println!("System Resource Monitor");
    println!("=====================");
    println!("Last update: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S").to_string().cyan());

    // CPU Table with color based on usage
    let usage_str = format!("{:.1}%", info.cpu_usage);
    let colored_usage = if info.cpu_usage >= 80.0 {
        usage_str.red().to_string()
    } else if info.cpu_usage >= 50.0 {
        usage_str.yellow().to_string()
    } else {
        usage_str.green().to_string()
    };
    
    let cpu_table = create_styled_table(CpuInfo {
        usage: colored_usage,
    });
    println!("{}\n", cpu_table);

    // Memory Table
    let memory_usage = (info.memory_used as f64 / info.memory_total as f64) * 100.0;
    let usage_str = format!("{:.1}%", memory_usage);
    let colored_usage = if memory_usage >= 85.0 {
        usage_str.red().to_string()
    } else if memory_usage >= 60.0 {
        usage_str.yellow().to_string()
    } else {
        usage_str.green().to_string()
    };

    let memory_table = create_styled_table_vec(vec![
        MemoryInfo {
            category: "Used".to_string(),
            value: format_bytes(info.memory_used),
        },
        MemoryInfo {
            category: "Total".to_string(),
            value: format_bytes(info.memory_total),
        },
        MemoryInfo {
            category: "Usage".to_string(),
            value: colored_usage,
        },
    ]);
    println!("{}\n", memory_table);

    // Temperature Table with color indicators
    if !info.components_temp.is_empty() {
        let temp_info: Vec<TempInfo> = info.components_temp
            .into_iter()
            .map(|(component, temp)| {
                let temp_str = format!("{:.1}°C", temp);
                let colored_temp = if temp >= 75.0 {
                    temp_str.red().to_string()
                } else if temp >= 55.0 {
                    temp_str.yellow().to_string()
                } else {
                    temp_str.green().to_string()
                };
                TempInfo {
                    component,
                    temp: colored_temp,
                }
            })
            .collect();
        
        let temp_table = create_styled_table_vec(temp_info);
        println!("{}", temp_table);
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
            // Clear current line and move cursor to start
            print!("\r\x1B[K");
            println!("\nMonitoring stopped. Thank you for using System Monitor!");
            io::stdout().flush().ok();
        })?;

        println!("System Resource Monitor");
        println!("=====================");
        println!("Press Ctrl+C to exit");
        println!("Updating every {} second{}\n", cli.interval, if cli.interval == 1 { "" } else { "s" });
        
        let mut is_first_run = true;
        let mut previous_height = None;
        while running.load(Ordering::SeqCst) {
            match display_system_info(is_first_run, previous_height) {
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
        if let Err(e) = display_system_info(true, None) {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }

    Ok(())
}
