# System Information Monitor

A fast and efficient system resource monitor written in Rust. Provides real-time monitoring of CPU usage, memory utilization, and component temperatures with color-coded status indicators.

## Features

- Real-time system resource monitoring
- Color-coded status indicators for quick visual assessment
- Clean, non-scrolling display updates
- Watch mode with customizable update intervals
- Modern table-based display format

## Usage

```bash
# Single snapshot of system information
cargo run

# Watch mode with default 1-second interval
cargo run -- --watch

# Watch mode with custom interval (e.g., 5 seconds)
cargo run -- --watch -n 5
```

## Status Indicators

The monitor uses color-coded indicators to help quickly identify system status:

### CPU Usage
- 🟢 Green: < 50% (Normal operation)
- 🟡 Yellow: ≥ 50% (Moderate load)
- 🔴 Red: ≥ 80% (High load)

### Memory Usage
- 🟢 Green: < 60% (Sufficient memory)
- 🟡 Yellow: ≥ 60% (Monitor memory usage)
- 🔴 Red: ≥ 85% (Low memory warning)

### Component Temperatures
- 🟢 Green: < 55°C (Normal temperature)
- 🟡 Yellow: ≥ 55°C (Monitor temperature)
- 🔴 Red: ≥ 75°C (High temperature warning)

## Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build for release
cargo build --release
```

## Dependencies

- sysinfo: System information retrieval
- clap: Command-line argument parsing
- tabled: Table formatting
- colored: Terminal colors
- chrono: Time formatting
- ctrlc: Signal handling

## License

MIT
