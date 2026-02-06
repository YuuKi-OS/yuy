use anyhow::Result;
use colored::Colorize;
use crate::config::{get_models_dir, YUUKI_MODELS, HF_ORG};

pub async fn execute(model: &str, variants: bool) -> Result<()> {
    println!("{}", "ℹ  Model Information".bright_cyan().bold());
    println!();

    if !YUUKI_MODELS.contains(&model) {
        anyhow::bail!("Model '{}' not found. Use 'yuy list models --remote' to see available models.", model);
    }

    println!("{} {}", "Model:".bright_cyan().bold(), model.bright_yellow().bold());
    println!(
        "{} https://huggingface.co/{}/{}",
        "URL:".bright_cyan(),
        HF_ORG,
        model
    );
    println!();

    // Check local status
    let models_dir = get_models_dir()?;
    let model_dir = models_dir.join(model);

    if model_dir.exists() {
        println!("{} {}", "Status:".bright_cyan(), "Downloaded ✓".bright_green());
        println!(
            "{} {}",
            "Location:".bright_cyan(),
            model_dir.display().to_string().bright_yellow()
        );
        println!();

        // List local variants
        println!("{}", "Local Variants:".bright_cyan());
        if let Ok(entries) = std::fs::read_dir(&model_dir) {
            for entry in entries {
                if let Ok(file) = entry {
                    let filename = file.file_name();
                    if filename.to_string_lossy().ends_with(".gguf") {
                        let metadata = file.metadata()?;
                        let size = crate::utils::format_size(metadata.len());
                        println!(
                            "  {} {} ({})",
                            "•".bright_green(),
                            filename.to_string_lossy().bright_white(),
                            size.bright_black()
                        );
                    }
                }
            }
        }
    } else {
        println!("{} {}", "Status:".bright_cyan(), "Not downloaded".bright_yellow());
        println!(
            "\n{} Download with: {}",
            "→".bright_blue(),
            format!("yuy download {}", model).bright_green()
        );
    }

    if variants {
        println!();
        println!("{}", "Available Variants (on HuggingFace):".bright_cyan());
        println!("  {} {} - Smallest size, good for mobile", "•".bright_green(), "q4_0".bright_white());
        println!("  {} {} - Medium quality, balanced", "•".bright_green(), "q5_k_m".bright_white());
        println!("  {} {} - High quality", "•".bright_green(), "q8_0".bright_white());
        println!("  {} {} - Full precision (largest)", "•".bright_green(), "f32".bright_white());
    }

    Ok(())
}
