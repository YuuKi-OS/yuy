use anyhow::Result;
use colored::Colorize;
use std::process::Command;
use crate::cli::RuntimeAction;
use crate::utils::{command_exists, detect_platform, Platform};

pub async fn execute(action: RuntimeAction) -> Result<()> {
    match action {
        RuntimeAction::Check => check_runtimes().await,
        RuntimeAction::Install { runtime } => install_runtime(runtime).await,
        RuntimeAction::List => list_runtimes().await,
    }
}

async fn check_runtimes() -> Result<()> {
    println!("{}", "🔍 Runtime Check".bright_cyan().bold());
    println!();

    // Check llama.cpp
    let llama_installed = command_exists("llama-cli")
        || command_exists("llama")
        || command_exists("main");

    if llama_installed {
        println!("{} {}", "✓".bright_green(), "llama.cpp".bright_white().bold());
        
        // Try to get version
        if command_exists("llama-cli") {
            if let Ok(output) = Command::new("llama-cli").arg("--version").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    println!("  {} {}", "→".bright_blue(), version.trim().bright_black());
                }
            }
        }
    } else {
        println!("{} {}", "✗".bright_red(), "llama.cpp".bright_white().bold());
        println!("  {} Not installed", "→".bright_black());
    }

    println!();

    // Check ollama
    let ollama_installed = command_exists("ollama");

    if ollama_installed {
        println!("{} {}", "✓".bright_green(), "ollama".bright_white().bold());
        
        if let Ok(output) = Command::new("ollama").arg("--version").output() {
            if let Ok(version) = String::from_utf8(output.stdout) {
                println!("  {} {}", "→".bright_blue(), version.trim().bright_black());
            }
        }
    } else {
        println!("{} {}", "✗".bright_red(), "ollama".bright_white().bold());
        println!("  {} Not installed", "→".bright_black());
    }

    println!();

    if !llama_installed && !ollama_installed {
        println!("{} No runtimes installed!", "⚠".bright_yellow());
        println!();
        println!("{} Install a runtime:", "→".bright_blue());
        println!("  {}", "yuy runtime install".bright_green());
    }

    Ok(())
}

async fn install_runtime(runtime: Option<String>) -> Result<()> {
    println!("{}", "📦 Runtime Installation".bright_cyan().bold());
    println!();

    let platform = detect_platform();
    
    let runtime_name = if let Some(r) = runtime {
        r
    } else {
        // Interactive selection
        println!("{} Select a runtime to install:", "?".bright_cyan());
        println!("  {} llama.cpp (recommended, lighter)", "1.".bright_white());
        println!("  {} ollama (more features, heavier)", "2.".bright_white());
        println!();
        print!("{} Enter choice [1/2]: ", "?".bright_cyan());
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => "llama-cpp".to_string(),
            "2" => "ollama".to_string(),
            _ => {
                println!("{} Invalid choice", "✗".bright_red());
                return Ok(());
            }
        }
    };

    println!();
    println!(
        "{} Installing: {}",
        "→".bright_blue(),
        runtime_name.bright_green().bold()
    );
    println!();

    match platform {
        Platform::Termux => install_on_termux(&runtime_name).await,
        Platform::Linux => install_on_linux(&runtime_name).await,
        Platform::MacOS => install_on_macos(&runtime_name).await,
        Platform::Windows => install_on_windows(&runtime_name).await,
        Platform::Unknown => {
            println!("{} Platform not supported for auto-install", "✗".bright_red());
            show_manual_instructions(&runtime_name, platform);
            Ok(())
        }
    }
}

async fn install_on_termux(runtime: &str) -> Result<()> {
    let package = match runtime {
        "llama-cpp" => "llama-cpp",
        "ollama" => "ollama",
        _ => anyhow::bail!("Unknown runtime: {}", runtime),
    };

    println!("{} Running: pkg install {}", "→".bright_blue(), package);
    println!();

    let status = Command::new("pkg")
        .arg("install")
        .arg(package)
        .arg("-y")
        .status()?;

    if status.success() {
        println!();
        println!(
            "{} {} installed successfully!",
            "✓".bright_green(),
            runtime.bright_green()
        );
    } else {
        anyhow::bail!("Installation failed");
    }

    Ok(())
}

async fn install_on_linux(runtime: &str) -> Result<()> {
    println!("{} Linux installation instructions:", "ℹ".bright_blue());
    show_manual_instructions(runtime, Platform::Linux);
    Ok(())
}

async fn install_on_macos(runtime: &str) -> Result<()> {
    println!("{} macOS installation:", "ℹ".bright_blue());
    println!();

    if command_exists("brew") {
        println!("{} Homebrew detected, attempting install...", "→".bright_blue());
        println!();

        let package = match runtime {
            "llama-cpp" => "llama.cpp",
            "ollama" => "ollama",
            _ => anyhow::bail!("Unknown runtime: {}", runtime),
        };

        let status = Command::new("brew")
            .arg("install")
            .arg(package)
            .status()?;

        if status.success() {
            println!();
            println!(
                "{} {} installed successfully!",
                "✓".bright_green(),
                runtime.bright_green()
            );
        } else {
            anyhow::bail!("Installation failed");
        }
    } else {
        println!("{} Homebrew not found", "✗".bright_red());
        show_manual_instructions(runtime, Platform::MacOS);
    }

    Ok(())
}

async fn install_on_windows(_runtime: &str) -> Result<()> {
    println!("{} Windows installation:", "ℹ".bright_blue());
    println!();
    println!("{} Automatic installation not yet supported on Windows", "⚠".bright_yellow());
    println!();
    println!("{} Manual installation:", "→".bright_blue());
    println!("  1. Install Chocolatey: https://chocolatey.org/install");
    println!("  2. Run: choco install llama-cpp  (or ollama)");
    println!();
    println!("{} Or download binaries:", "→".bright_blue());
    println!("  • llama.cpp: https://github.com/ggerganov/llama.cpp/releases");
    println!("  • ollama: https://ollama.com/download");

    Ok(())
}

fn show_manual_instructions(runtime: &str, platform: Platform) {
    println!();
    println!("{}", "Manual Installation:".bright_cyan().bold());
    println!();

    match (runtime, platform) {
        ("llama-cpp", Platform::Linux) => {
            println!("{}  Option 1 - Download binary:", "1.".bright_white());
            println!("   {}", "https://github.com/ggerganov/llama.cpp/releases".bright_blue());
            println!();
            println!("{}  Option 2 - Build from source:", "2.".bright_white());
            println!("   git clone https://github.com/ggerganov/llama.cpp");
            println!("   cd llama.cpp && make");
        }
        ("ollama", Platform::Linux) => {
            println!("{}  Run the install script:", "→".bright_blue());
            println!("   curl -fsSL https://ollama.com/install.sh | sh");
        }
        _ => {
            println!("{} Visit the official website for instructions", "→".bright_blue());
        }
    }
}

async fn list_runtimes() -> Result<()> {
    println!("{}", "📋 Available Runtimes".bright_cyan().bold());
    println!();

    println!("{}", "llama.cpp".bright_green().bold());
    println!("  {} Fast, lightweight C++ inference engine", "→".bright_blue());
    println!("  {} Best for: Termux, low-end devices, direct CLI usage", "→".bright_blue());
    println!("  {} https://github.com/ggerganov/llama.cpp", "🔗".bright_blue());
    println!();

    println!("{}", "ollama".bright_green().bold());
    println!("  {} User-friendly model management with server", "→".bright_blue());
    println!("  {} Best for: Desktop usage, multiple models, API access", "→".bright_blue());
    println!("  {} https://ollama.com", "🔗".bright_blue());
    println!();

    println!("{} Install a runtime:", "→".bright_blue());
    println!("  {}", "yuy runtime install".bright_white());

    Ok(())
}
