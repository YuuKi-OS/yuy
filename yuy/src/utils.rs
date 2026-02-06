use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Termux,
    Unknown,
}

pub fn detect_platform() -> Platform {
    let os = std::env::consts::OS;
    
    match os {
        "linux" => {
            // Check if running in Termux
            if std::env::var("PREFIX").is_ok() && std::env::var("PREFIX").unwrap().contains("com.termux") {
                Platform::Termux
            } else {
                Platform::Linux
            }
        }
        "macos" => Platform::MacOS,
        "windows" => Platform::Windows,
        _ => Platform::Unknown,
    }
}

pub fn get_available_ram_gb() -> usize {
    // Simplified: assume 8GB if we can't detect
    // In production, use sys-info or similar
    8
}

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

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

pub fn recommend_quantization(platform: Platform, ram_gb: usize) -> &'static str {
    match (platform, ram_gb) {
        (Platform::Termux, _) => "q4_0",
        (_, ram) if ram < 8 => "q4_k_m",
        (_, ram) if ram < 16 => "q5_k_m",
        _ => "q8_0",
    }
}
