use anyhow::Result;
use colored::Colorize;
use crate::config::get_models_dir;

pub async fn execute(model: &str) -> Result<()> {
    println!("{}", "🗑️  Remove Model".bright_cyan().bold());
    println!();

    let models_dir = get_models_dir()?;
    let model_dir = models_dir.join(model);

    if !model_dir.exists() {
        println!(
            "{} Model '{}' is not downloaded.",
            "ℹ".bright_blue(),
            model.bright_yellow()
        );
        return Ok(());
    }

    // Calculate total size
    let mut total_size = 0u64;
    for entry in std::fs::read_dir(&model_dir)? {
        if let Ok(file) = entry {
            if let Ok(metadata) = file.metadata() {
                total_size += metadata.len();
            }
        }
    }

    println!(
        "{} About to remove: {}",
        "⚠".bright_yellow(),
        model.bright_yellow().bold()
    );
    println!(
        "{} Space to free: {}",
        "→".bright_blue(),
        crate::utils::format_size(total_size).bright_green()
    );
    println!();

    print!("{} Are you sure? [y/N]: ", "?".bright_cyan());
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("{} Cancelled.", "ℹ".bright_blue());
        return Ok(());
    }

    std::fs::remove_dir_all(&model_dir)?;

    println!();
    println!(
        "{} Model '{}' removed successfully.",
        "✓".bright_green(),
        model.bright_yellow()
    );
    println!(
        "{} Freed {} of space.",
        "→".bright_blue(),
        crate::utils::format_size(total_size).bright_green()
    );

    Ok(())
}
