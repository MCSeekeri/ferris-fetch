use clap::Parser;
use colored::*;
use crossterm::cursor::{MoveDown, MoveToColumn, MoveUp, RestorePosition, SavePosition};
use crossterm::execute;
use image::{DynamicImage, RgbaImage};
use resvg::{render, tiny_skia, usvg};
use std::env;
use std::io::{self, Write};
use sysinfo::System;
use tiny_skia::{Pixmap, Transform};

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
const FERRIS_SMALL: &[&str] = &[r"   _~^~_   ", r" \)/o o\(/ ", r"  '- ^ -'  "];

const FERRIS_SVG: &[u8] = include_bytes!("rustacean-flat-happy.svg");
const MIN_IMAGE_WIDTH: u16 = 10;
const MAX_IMAGE_WIDTH: u16 = 40;

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
                    .next_back()
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
            username: whoami::username().unwrap_or_else(|_| "Unknown".to_string()),
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

fn terminal_size() -> (u16, u16) {
    crossterm::terminal::size().unwrap_or((80, 24))
}

fn render_ferris() -> Result<DynamicImage, String> {
    let opt = usvg::Options::default();

    let tree = usvg::Tree::from_data(FERRIS_SVG, &opt).map_err(|err| err.to_string())?;
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or_else(|| "Failed to create pixmap".to_string())?;

    render(&tree, Transform::default(), &mut pixmap.as_mut());
    let image = pixmap_to_rgba(&pixmap)?;
    Ok(DynamicImage::ImageRgba8(image))
}

fn pixmap_to_rgba(pixmap: &Pixmap) -> Result<RgbaImage, String> {
    let width = pixmap.width();
    let height = pixmap.height();
    let pixels = pixmap.pixels();
    let mut raw = Vec::with_capacity((width * height * 4) as usize);

    for pixel in pixels {
        let color = pixel.demultiply();
        raw.push(color.red());
        raw.push(color.green());
        raw.push(color.blue());
        raw.push(color.alpha());
    }

    RgbaImage::from_raw(width, height, raw).ok_or_else(|| "Failed to build RGBA image".to_string())
}

fn print_ferris(image: &DynamicImage, max_width: Option<u32>) -> Result<(u32, u32), String> {
    let mut config = viuer::Config {
        use_kitty: !cfg!(windows),
        use_iterm: !cfg!(windows),
        ..Default::default()
    };
    config.transparent = true;
    config.premultiplied_alpha = false;
    config.absolute_offset = false;
    config.restore_cursor = false;
    config.width = max_width;

    let (width, height) = viuer::print(image, &config).map_err(|err| err.to_string())?;

    let mut stdout = io::stdout();
    execute!(stdout, MoveUp(height as u16)).map_err(|err: std::io::Error| err.to_string())?;

    Ok((width, height))
}

fn print_lines_with_offset(lines: &[String], offset: u16, art_height: u32) -> Result<(), String> {
    let mut stdout = io::stdout();
    execute!(stdout, MoveToColumn(0), SavePosition)
        .map_err(|err: std::io::Error| err.to_string())?;

    for (idx, line) in lines.iter().enumerate() {
        let row = (idx as u32).min(u16::MAX as u32) as u16;
        if row == 0 {
            execute!(stdout, RestorePosition, MoveToColumn(offset))
                .map_err(|err: std::io::Error| err.to_string())?;
        } else {
            execute!(stdout, RestorePosition, MoveDown(row), MoveToColumn(offset))
                .map_err(|err: std::io::Error| err.to_string())?;
        }
        writeln!(stdout, "{line}").map_err(|err: std::io::Error| err.to_string())?;
    }

    let final_height = art_height.max(lines.len() as u32);
    let final_height = final_height.min(u16::MAX as u32) as u16;
    execute!(
        stdout,
        RestorePosition,
        MoveDown(final_height),
        MoveToColumn(0)
    )
    .map_err(|err: std::io::Error| err.to_string())?;
    writeln!(stdout).map_err(|err: std::io::Error| err.to_string())?;
    stdout
        .flush()
        .map_err(|err: std::io::Error| err.to_string())?;
    Ok(())
}

/// Print system information with Ferris art
fn print_info(info: &SysInfo, args: &Args, theme: &Theme) {
    let mut lines: Vec<(String, String)> = Vec::new();
    let (term_cols, _) = terminal_size();
    let art: &[&str] = if args.minimal {
        FERRIS_SMALL
    } else {
        FERRIS_ART
    };
    let art_width = if args.no_art {
        0usize
    } else {
        art.iter().map(|l| l.len()).max().unwrap_or(0)
    };
    let info_cols = if args.no_art {
        term_cols as usize
    } else {
        term_cols.saturating_sub((art_width + 2) as u16) as usize
    };

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
    let sep = "─".repeat(title.chars().count());
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
        let cpu_suffix = format!(" ({} cores)", info.cpu_cores);
        let max_cpu_len = info_cols
            .saturating_sub("CPU: ".len() + cpu_suffix.chars().count())
            .max(4);
        let cpu_display = if info.cpu.chars().count() > max_cpu_len {
            let mut trimmed = info
                .cpu
                .chars()
                .take(max_cpu_len.saturating_sub(3))
                .collect::<String>();
            trimmed.push_str("...");
            trimmed
        } else {
            info.cpu.clone()
        };
        let cpu_line = format!("{cpu_display}{cpu_suffix}");
        lines.push((label_color("CPU"), value_color(&cpu_line)));

        // Memory with bar
        let mem_info = format!(
            "{} / {} {}",
            format_bytes(info.memory_used),
            format_bytes(info.memory_total),
            progress_bar(
                info.memory_used,
                info.memory_total,
                10,
                theme,
                args.no_color
            )
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

    let line_strings: Vec<String> = lines
        .iter()
        .map(|(label, value)| {
            if label.is_empty() {
                value.clone()
            } else {
                format!("{label}: {value}")
            }
        })
        .collect();

    let max_info_width = line_strings
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;
    let available_width = term_cols.saturating_sub(max_info_width + 2);
    let max_image_width = if available_width >= MIN_IMAGE_WIDTH {
        Some(available_width.min(MAX_IMAGE_WIDTH) as u32)
    } else {
        None
    };

    if !args.no_art
        && !args.minimal
        && let Some(max_width) = max_image_width
        && let Ok(image) = render_ferris()
        && let Ok((width, height)) = print_ferris(&image, Some(max_width))
    {
        let offset = width
            .saturating_add(2)
            .min(term_cols as u32)
            .min(u16::MAX as u32) as u16;
        if print_lines_with_offset(&line_strings, offset, height).is_ok() {
            return;
        }
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
