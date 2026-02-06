use anyhow::Result;
use colored::Colorize;
use crate::cli::ListTarget;
use crate::config::{get_models_dir, YUUKI_MODELS, HF_ORG};

pub async fn execute(target: ListTarget) -> Result<()> {
    match target {
        ListTarget::Models { remote } => {
            if remote {
                list_remote_models().await
            } else {
                list_local_models().await
            }
        }
    }
}

async fn list_local_models() -> Result<()> {
    println!("{}", "📋 Local Models".bright_cyan().bold());
    println!();

    let models_dir = get_models_dir()?;

    if !models_dir.exists() || std::fs::read_dir(&models_dir)?.next().is_none() {
        println!("{} No models downloaded yet.", "ℹ".bright_blue());
        println!();
        println!("{} Download a model:", "→".bright_blue());
        println!("  {}", "yuy download Yuuki-best".bright_green());
        return Ok(());
    }

    for entry in std::fs::read_dir(&models_dir)? {
        let entry = entry?;
        let model_name = entry.file_name();
        let model_path = entry.path();

        if model_path.is_dir() {
            println!("{} {}", "•".bright_green(), model_name.to_string_lossy().bright_yellow().bold());

            // List GGUF files in this model directory
            if let Ok(files) = std::fs::read_dir(&model_path) {
                for file_entry in files {
                    if let Ok(file) = file_entry {
                        let file_name = file.file_name();
                        if file_name.to_string_lossy().ends_with(".gguf") {
                            let metadata = file.metadata()?;
                            let size = crate::utils::format_size(metadata.len());
                            println!(
                                "  {} {} ({})",
                                "→".bright_blue(),
                                file_name.to_string_lossy().bright_white(),
                                size.bright_black()
                            );
                        }
                    }
                }
            }
            println!();
        }
    }

    println!(
        "{} Location: {}",
        "📁".bright_blue(),
        models_dir.display().to_string().bright_black()
    );

    Ok(())
}

async fn list_remote_models() -> Result<()> {
    println!("{}", "🌐 Available Models (Hugging Face)".bright_cyan().bold());
    println!();

    println!(
        "{} Organization: {}",
        "→".bright_blue(),
        HF_ORG.bright_yellow()
    );
    println!();

    for model in YUUKI_MODELS {
        println!("{} {}", "•".bright_green(), model.bright_yellow().bold());
        println!(
            "  {} https://huggingface.co/{}/{}",
            "🔗".bright_blue(),
            HF_ORG,
            model
        );
        println!(
            "  {} {}",
            "📥".bright_blue(),
            format!("yuy download {}", model).bright_green()
        );
        println!();
    }

    println!("{} Quantizations typically available:", "ℹ".bright_blue());
    println!("  • {} (smallest, fastest)", "q4_0".bright_green());
    println!("  • {} (balanced)", "q5_k_m".bright_green());
    println!("  • {} (best quality)", "q8_0".bright_green());
    println!("  • {} (full precision)", "f32".bright_green());

    Ok(())
}
