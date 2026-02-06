use anyhow::Result;
use colored::Colorize;
use crate::config::{get_yuuki_dir, get_models_dir};
use crate::utils::{command_exists, detect_platform};

pub async fn execute() -> Result<()> {
    println!("{}", "🌸 Yuuki Setup Wizard".bright_magenta().bold());
    println!();

    println!("{}", "Welcome to Yuuki! Let's get you set up.".bright_cyan());
    println!();

    // Step 1: Create directories
    println!("{} Creating directories...", "1.".bright_white().bold());
    let yuuki_dir = get_yuuki_dir()?;
    let models_dir = get_models_dir()?;

    println!(
        "  {} {}",
        "✓".bright_green(),
        yuuki_dir.display().to_string().bright_yellow()
    );
    println!(
        "  {} {}",
        "✓".bright_green(),
        models_dir.display().to_string().bright_yellow()
    );
    println!();

    // Step 2: Check platform
    println!("{} Detecting platform...", "2.".bright_white().bold());
    let platform = detect_platform();
    println!("  {} {:?}", "→".bright_blue(), platform);
    println!();

    // Step 3: Check runtimes
    println!("{} Checking for runtimes...", "3.".bright_white().bold());
    
    let llama_installed = command_exists("llama-cli")
        || command_exists("llama")
        || command_exists("main");
    let ollama_installed = command_exists("ollama");

    if llama_installed {
        println!("  {} llama.cpp found", "✓".bright_green());
    } else {
        println!("  {} llama.cpp not found", "✗".bright_yellow());
    }

    if ollama_installed {
        println!("  {} ollama found", "✓".bright_green());
    } else {
        println!("  {} ollama not found", "✗".bright_yellow());
    }

    println!();

    // Step 4: Offer to install runtime
    if !llama_installed && !ollama_installed {
        println!("{} No runtime detected.", "⚠".bright_yellow());
        println!();
        print!("{} Would you like to install a runtime now? [y/N]: ", "?".bright_cyan());
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            println!();
            crate::commands::runtime::execute(crate::cli::RuntimeAction::Install { runtime: None }).await?;
        } else {
            println!();
            println!("{} Skipping runtime installation.", "→".bright_blue());
            println!(
                "{} You can install later with: {}",
                "→".bright_blue(),
                "yuy runtime install".bright_green()
            );
        }
    } else {
        println!("{} Runtime ready!", "✓".bright_green());
    }

    println!();

    // Step 5: Summary
    println!("{}", "✨ Setup Complete!".bright_green().bold());
    println!();
    println!("{}", "Next steps:".bright_cyan());
    println!("  {} Download a model:", "1.".bright_white());
    println!("     {}", "yuy download Yuuki-best".bright_green());
    println!();
    println!("  {} Run the model:", "2.".bright_white());
    println!("     {}", "yuy run Yuuki-best".bright_green());
    println!();
    println!("  {} Check system health:", "3.".bright_white());
    println!("     {}", "yuy doctor".bright_green());
    println!();

    println!("{} Need help? Visit:", "📚".bright_blue());
    println!("  https://github.com/YuuKi-OS/yuy");

    Ok(())
}
