# Overtime Tracker

A simple and fast CLI tool for tracking overtime hours. This lightweight application allows you to add, remove, and list your overtime with a minimal footprint.

## Features

- Track overtime hours and minutes with a simple command-line interface
- Add and remove overtime entries by date
- List all overtime entries with a monthly breakdown
- View total accumulated overtime
- Stores data in a user-specific local data directory

## Installation

### From Source

1. Make sure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs/)
2. Clone this repository or download the source code
3. Navigate to the project directory
4. Run:

```bash
cargo install --path .
```

This will compile the application in release mode and install it to your system.

## Usage

### Adding Overtime

To add overtime for a specific date:

```bash
overtime add <time> <date>
```

Examples:
```bash
overtime add 1h 25042025      # Add 1 hour for April 25, 2025
overtime add 1h30m 25042025   # Add 1 hour and 30 minutes for April 25, 2025
overtime add 45m 25.04.2025   # Add 45 minutes for April 25, 2025
```

### Removing Overtime

To remove overtime for a specific date:

```bash
overtime remove <time> <date>
```

Examples:
```bash
overtime remove 1h 25042025      # Remove 1 hour for April 25, 2025
overtime remove 30m 25.04.2025   # Remove 30 minutes for April 25, 2025
```

### Listing Overtime

To view all overtime entries and total accumulated overtime:

```bash
overtime list
```

This will display a monthly breakdown of all entries and a total summary.

## Time Format

The time parameter accepts the following formats:
- `h` or `H` for hours: `1h`, `2H`
- `m` or `M` for minutes: `30m`, `45M`
- Combined hours and minutes: `1h30m`, `2H15M`

## Date Format

The date parameter accepts the following formats:
- `ddmmYYYY`: `25042025` (April 25, 2025)
- `dd.mm.YYYY`: `25.04.2025` (April 25, 2025)

## Output Example

```
2025-4:
Date       | Overtime
-----------+---------
25.04.2025 |  1h 30m
26.04.2025 |  2h 0m
27.04.2025 | -1h 0m

Total overtime: 2 hours and 30 minutes
```

## Data Storage

All overtime data is stored in a JSON file in your user's local data directory:
- Linux: `~/.local/share/overtime/overtime.json`
