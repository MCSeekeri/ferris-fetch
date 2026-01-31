//! # ferris-fetch 🦀
//!
//! A fast and cute system information tool written in Rust, featuring Ferris the crab!
//!
//! ## Features
//! - Pure Rust implementation, blazingly fast startup
//! - Cross-platform support (Windows, Linux, macOS)
//! - Customizable color themes
//! - ASCII art Ferris mascot
//!
//! ## Usage
//! ```bash
//! ferris-fetch           # Default display
//! ferris-fetch --theme ocean   # Use ocean theme
//! ferris-fetch --no-color      # Disable colors
//! ```

use clap::Parser;
use colored::*;
use std::env;
use sysinfo::System;

/// Ferris-fetch: A cute system information tool 🦀
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Color theme to use (rust, ocean, forest, sunset, mono)
    #[arg(short, long, default_value = "rust")]
    theme: String,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Show minimal info only
    #[arg(short, long)]
    minimal: bool,

    /// Hide ASCII art
    #[arg(long)]
    no_art: bool,
}

/// Color theme configuration
#[derive(Clone)]
struct Theme {
    primary: Color,
    secondary: Color,
    accent: Color,
    info: Color,
}

impl Theme {
    fn get(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "ocean" => Theme {
                primary: Color::Cyan,
                secondary: Color::Blue,
                accent: Color::BrightCyan,
                info: Color::White,
            },
            "forest" => Theme {
                primary: Color::Green,
                secondary: Color::BrightGreen,
                accent: Color::Yellow,
                info: Color::White,
            },
            "sunset" => Theme {
                primary: Color::Red,
                secondary: Color::Yellow,
                accent: Color::Magenta,
                info: Color::White,
            },
            "mono" => Theme {
                primary: Color::White,
                secondary: Color::White,
                accent: Color::White,
                info: Color::White,
            },
            _ => Theme {
                // Default "rust" theme
                primary: Color::TrueColor {
                    r: 255,
                    g: 128,
                    b: 0,
                }, // Rust orange
                secondary: Color::TrueColor {
                    r: 183,
                    g: 65,
                    b: 14,
                }, // Rust brown
                accent: Color::TrueColor {
                    r: 255,
                    g: 200,
                    b: 100,
                },
                info: Color::White,
            },
        }
    }
}

/// ASCII art for Ferris the crab
const FERRIS_ART: &[&str] = &[
    r"        _~^~^~_        ",
    r"    \) /  o o  \ (/    ",
    r"      '_   -   _'      ",
    r"      / '-----' \      ",
    r"     /           \     ",
    r"    /  /       \  \    ",
    r"   (  |         |  )   ",
    r"    \_|         |_/    ",
];

/// Small Ferris for minimal mode
const FERRIS_SMALL: &[&str] = &[
    r"   _~^~_   ",
    r" \)/o o\(/ ",
    r"  '- ^ -'  ",
];

/// System information collector
struct SysInfo {
    hostname: String,
    username: String,
    os: String,
    kernel: String,
    uptime: String,
    shell: String,
    cpu: String,
    cpu_cores: usize,
    memory_used: u64,
    memory_total: u64,
    #[allow(dead_code)]
    disk_used: u64,
    #[allow(dead_code)]
    disk_total: u64,
}

impl SysInfo {
    fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Get OS info
        let os_info = os_info::get();
        let os = format!("{} {}", os_info.os_type(), os_info.version());

        // Get kernel version
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        // Get uptime
        let uptime_secs = System::uptime();
        let uptime = format_uptime(uptime_secs);

        // Get shell
        let shell = env::var("SHELL")
            .or_else(|_| env::var("COMSPEC"))
            .map(|s| {
                s.split(['/', '\\'])
                    .last()
                    .unwrap_or("Unknown")
                    .to_string()
            })
            .unwrap_or_else(|_| "Unknown".to_string());

        // Get CPU info
        let cpu = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let cpu_cores = sys.cpus().len();

        // Get memory info
        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();

        // Get disk info
        let mut disk_total = 0u64;
        let mut disk_used = 0u64;
        for disk in sysinfo::Disks::new_with_refreshed_list().iter() {
            disk_total += disk.total_space();
            disk_used += disk.total_space() - disk.available_space();
        }

        SysInfo {
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            username: whoami::username(),
            os,
            kernel,
            uptime,
            shell,
            cpu,
            cpu_cores,
            memory_used,
            memory_total,
            disk_used,
            disk_total,
        }
    }
}

/// Format uptime in human readable format
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if mins > 0 || parts.is_empty() {
        parts.push(format!("{}m", mins));
    }
    parts.join(" ")
}

/// Format bytes to human readable size
fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    }
}

/// Create a progress bar
fn progress_bar(used: u64, total: u64, width: usize, theme: &Theme, no_color: bool) -> String {
    let percentage = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u8
    } else {
        0
    };
    let filled = (percentage as usize * width) / 100;
    let empty = width - filled;

    let bar = format!(
        "[{}{}] {}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percentage
    );

    if no_color {
        bar
    } else {
        let color = if percentage > 80 {
            Color::Red
        } else if percentage > 60 {
            Color::Yellow
        } else {
            theme.accent
        };
        bar.color(color).to_string()
    }
}

/// Print system information with Ferris art
fn print_info(info: &SysInfo, args: &Args, theme: &Theme) {
    let art: &[&str] = if args.minimal {
        FERRIS_SMALL
    } else {
        FERRIS_ART
    };
    let art_width = art.iter().map(|l| l.len()).max().unwrap_or(0);

    // Prepare info lines
    let mut lines: Vec<(String, String)> = Vec::new();

    // Title line
    let title = format!("{}@{}", info.username, info.hostname);
    lines.push((
        "".to_string(),
        if args.no_color {
            title.clone()
        } else {
            title.color(theme.primary).bold().to_string()
        },
    ));

    // Separator
    let sep = "─".repeat(title.len());
    lines.push((
        "".to_string(),
        if args.no_color {
            sep
        } else {
            sep.color(theme.secondary).to_string()
        },
    ));

    // System info
    let label_color = |s: &str| -> String {
        if args.no_color {
            s.to_string()
        } else {
            s.color(theme.primary).bold().to_string()
        }
    };

    let value_color = |s: &str| -> String {
        if args.no_color {
            s.to_string()
        } else {
            s.color(theme.info).to_string()
        }
    };

    lines.push((label_color("OS"), value_color(&info.os)));
    lines.push((label_color("Kernel"), value_color(&info.kernel)));
    lines.push((label_color("Uptime"), value_color(&info.uptime)));
    lines.push((label_color("Shell"), value_color(&info.shell)));

    if !args.minimal {
        // CPU info with cores
        let cpu_display = if info.cpu.len() > 35 {
            format!("{}...", &info.cpu[..32])
        } else {
            info.cpu.clone()
        };
        lines.push((
            label_color("CPU"),
            value_color(&format!("{} ({} cores)", cpu_display, info.cpu_cores)),
        ));

        // Memory with bar
        let mem_info = format!(
            "{} / {} {}",
            format_bytes(info.memory_used),
            format_bytes(info.memory_total),
            progress_bar(info.memory_used, info.memory_total, 10, theme, args.no_color)
        );
        lines.push((label_color("Memory"), mem_info));
    }

    // Empty line before color blocks
    lines.push(("".to_string(), "".to_string()));

    // Color palette
    if !args.minimal && !args.no_color {
        let palette: String = [
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
        ]
        .iter()
        .map(|c| "███".color(*c).to_string())
        .collect();
        lines.push(("".to_string(), palette));

        let bright_palette: String = [
            Color::BrightBlack,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightYellow,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightCyan,
            Color::BrightWhite,
        ]
        .iter()
        .map(|c| "███".color(*c).to_string())
        .collect();
        lines.push(("".to_string(), bright_palette));
    }

    // Print output
    let max_lines = art.len().max(lines.len());

    for i in 0..max_lines {
        // Print art part
        if !args.no_art {
            let art_line = if i < art.len() { art[i] } else { "" };
            let colored_art = if args.no_color {
                format!("{:width$}", art_line, width = art_width)
            } else {
                format!("{:width$}", art_line, width = art_width)
                    .color(theme.primary)
                    .to_string()
            };
            print!("{}  ", colored_art);
        }

        // Print info part
        if i < lines.len() {
            let (label, value) = &lines[i];
            if label.is_empty() {
                println!("{}", value);
            } else {
                println!("{}: {}", label, value);
            }
        } else {
            println!();
        }
    }
}

fn main() {
    let args = Args::parse();

    // Disable colors if requested or if terminal doesn't support it
    if args.no_color {
        colored::control::set_override(false);
    }

    // Get theme
    let theme = Theme::get(&args.theme);

    // Collect system info
    let info = SysInfo::collect();

    // Print the info
    print_info(&info, &args, &theme);
}
