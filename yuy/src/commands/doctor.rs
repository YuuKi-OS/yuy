use anyhow::Result;
use colored::Colorize;
use crate::config::{get_yuuki_dir, get_models_dir};
use crate::utils::{command_exists, detect_platform, get_available_ram_gb, recommend_quantization};

pub async fn execute() -> Result<()> {
    println!("{}", "🔍 Yuuki System Doctor".bright_cyan().bold());
    println!();

    // Platform
    let platform = detect_platform();
    println!("{}", "System Information:".bright_cyan());
    println!(
        "  {} {:?}",
        "Platform:".bright_white(),
        platform
    );
    println!(
        "  {} {}",
        "OS:".bright_white(),
        std::env::consts::OS
    );
    println!(
        "  {} {}",
        "Arch:".bright_white(),
        std::env::consts::ARCH
    );

    // RAM
    let ram = get_available_ram_gb();
    println!(
        "  {} ~{} GB",
        "RAM:".bright_white(),
        ram
    );

    // Recommended quantization
    let recommended_quant = recommend_quantization(platform.clone(), ram);
    println!(
        "  {} {}",
        "Recommended quantization:".bright_white(),
        recommended_quant.bright_green()
    );

    println!();

    // Yuuki directories
    println!("{}", "Yuuki Configuration:".bright_cyan());
    
    let yuuki_dir = get_yuuki_dir()?;
    println!(
        "  {} {}",
        "Config dir:".bright_white(),
        yuuki_dir.display().to_string().bright_yellow()
    );

    let models_dir = get_models_dir()?;
    println!(
        "  {} {}",
        "Models dir:".bright_white(),
        models_dir.display().to_string().bright_yellow()
    );

    // Check disk space
    if models_dir.exists() {
        let mut total_size = 0u64;
        let mut model_count = 0;

        for entry in std::fs::read_dir(&models_dir)? {
            if let Ok(dir_entry) = entry {
                if dir_entry.path().is_dir() {
                    model_count += 1;
                    // Calculate size
                    if let Ok(files) = std::fs::read_dir(dir_entry.path()) {
                        for file in files {
                            if let Ok(f) = file {
                                if let Ok(metadata) = f.metadata() {
                                    total_size += metadata.len();
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "  {} {}",
            "Models downloaded:".bright_white(),
            model_count
        );
        println!(
            "  {} {}",
            "Total size:".bright_white(),
            crate::utils::format_size(total_size).bright_green()
        );
    }

    println!();

    // Runtime status
    println!("{}", "Runtime Status:".bright_cyan());

    let llama_installed = command_exists("llama-cli")
        || command_exists("llama")
        || command_exists("main");

    if llama_installed {
        println!("  {} {} {}", "✓".bright_green(), "llama.cpp".bright_white(), "installed".bright_green());
    } else {
        println!("  {} {} {}", "✗".bright_red(), "llama.cpp".bright_white(), "not installed".bright_red());
    }

    let ollama_installed = command_exists("ollama");

    if ollama_installed {
        println!("  {} {} {}", "✓".bright_green(), "ollama".bright_white(), "installed".bright_green());
    } else {
        println!("  {} {} {}", "✗".bright_red(), "ollama".bright_white(), "not installed".bright_red());
    }

    println!();

    // Dependencies
    println!("{}", "System Dependencies:".bright_cyan());
    
    check_command("curl");
    check_command("wget");
    check_command("git");

    println!();

    // Health summary
    println!("{}", "Health Summary:".bright_cyan().bold());
    let mut issues = Vec::new();

    if !llama_installed && !ollama_installed {
        issues.push("No runtime installed");
    }

    if issues.is_empty() {
        println!(
            "  {} System is ready to use Yuuki!",
            "✓".bright_green().bold()
        );
    } else {
        println!(
            "  {} {} issue(s) found:",
            "⚠".bright_yellow(),
            issues.len()
        );
        for issue in issues {
            println!("    • {}", issue.bright_yellow());
        }
        println!();
        println!(
            "{} Run {} to install a runtime",
            "→".bright_blue(),
            "yuy runtime install".bright_green()
        );
    }

    Ok(())
}

fn check_command(cmd: &str) {
    if command_exists(cmd) {
        println!("  {} {} {}", "✓".bright_green(), cmd.bright_white(), "available".bright_green());
    } else {
        println!("  {} {} {}", "✗".bright_yellow(), cmd.bright_white(), "not found".bright_yellow());
    }
}
