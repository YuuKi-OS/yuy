use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use crate::config::{get_models_dir, HF_ORG, YUUKI_MODELS};
use crate::utils::{detect_platform, get_available_ram_gb, recommend_quantization};

pub async fn execute(model: &str, quant: Option<String>) -> Result<()> {
    println!("{}", "📥 Yuuki Model Downloader".bright_cyan().bold());
    println!();

    // Validate model name
    if !YUUKI_MODELS.contains(&model) {
        println!(
            "{} Model '{}' not found",
            "✗".bright_red(),
            model.bright_yellow()
        );
        println!("\n{}", "Available models:".bright_cyan());
        for m in YUUKI_MODELS {
            println!("  • {}", m.bright_green());
        }
        return Ok(());
    }

    // Determine quantization
    let quantization = if let Some(q) = quant {
        q
    } else {
        let platform = detect_platform();
        let ram = get_available_ram_gb();
        let recommended = recommend_quantization(platform, ram);
        println!(
            "{} Auto-selected quantization: {} (based on your system)",
            "ℹ".bright_blue(),
            recommended.bright_green()
        );
        recommended.to_string()
    };

    println!(
        "{} Model: {}",
        "→".bright_blue(),
        model.bright_green().bold()
    );
    println!(
        "{} Quantization: {}",
        "→".bright_blue(),
        quantization.bright_green()
    );
    println!(
        "{} Source: {}/{}",
        "→".bright_blue(),
        HF_ORG.bright_yellow(),
        model.bright_yellow()
    );
    println!();

    // Create model directory
    let models_dir = get_models_dir()?;
    let model_dir = models_dir.join(model);
    std::fs::create_dir_all(&model_dir)?;

    // Construct Hugging Face URL
    let filename = format!("{}-{}.gguf", model.to_lowercase(), quantization);
    let url = format!(
        "https://huggingface.co/{}/{}/resolve/main/{}",
        HF_ORG, model, filename
    );

    println!("{} Downloading from Hugging Face...", "↓".bright_cyan());
    println!("{} URL: {}", "  ".bright_black(), url.bright_black());
    println!();

    // Download file with progress bar
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to start download")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download: HTTP {} - Model file might not exist yet. Try checking HuggingFace.",
            response.status()
        );
    }

    let total_size = response
        .content_length()
        .context("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let output_path = model_dir.join(&filename);
    let mut file = File::create(&output_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.context("Error while downloading file")?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete!");
    println!();

    println!(
        "{} Model downloaded successfully!",
        "✓".bright_green().bold()
    );
    println!(
        "  {} {}",
        "Location:".bright_cyan(),
        output_path.display().to_string().bright_yellow()
    );
    println!(
        "  {} {}",
        "Size:".bright_cyan(),
        crate::utils::format_size(total_size).bright_yellow()
    );
    println!();
    println!(
        "{} Run the model with: {}",
        "→".bright_blue(),
        format!("yuy run {}", model).bright_green()
    );

    Ok(())
}
