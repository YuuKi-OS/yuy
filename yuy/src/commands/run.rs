use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;
use crate::config::{get_models_dir, YUUKI_MODELS, OLLAMA_ORG, HF_ORG};
use crate::utils::command_exists;

pub async fn execute(
    model: &str,
    runtime: Option<String>,
    quant: Option<String>,
    preset: Option<String>,
    _resume: bool,
    _template: Option<String>,
) -> Result<()> {
    println!("{}", "🚀 Yuuki Runtime".bright_cyan().bold());
    println!();

    // Validate model
    if !YUUKI_MODELS.contains(&model) {
        anyhow::bail!("Model '{}' not found. Use 'yuy list models' to see available models.", model);
    }

    // Check if model is downloaded
    let models_dir = get_models_dir()?;
    let model_dir = models_dir.join(model);
    
    if !model_dir.exists() {
        println!(
            "{} Model '{}' is not downloaded yet.",
            "✗".bright_red(),
            model.bright_yellow()
        );
        println!(
            "\n{} Download it first: {}",
            "→".bright_blue(),
            format!("yuy download {}", model).bright_green()
        );
        return Ok(());
    }

    // Find GGUF file
    let quant_str = quant.unwrap_or_else(|| "q5_k_m".to_string());
    let filename = format!("{}-{}.gguf", model.to_lowercase(), quant_str);
    let model_path = model_dir.join(&filename);

    if !model_path.exists() {
        anyhow::bail!(
            "Model file '{}' not found. Available quantizations may differ. Try: yuy info {}",
            filename, model
        );
    }

    println!("{} Model: {}", "→".bright_blue(), model.bright_green());
    println!("{} File: {}", "→".bright_blue(), filename.bright_yellow());
    if let Some(p) = &preset {
        println!("{} Preset: {}", "→".bright_blue(), p.bright_magenta());
    }
    println!();

    // Determine runtime
    let runtime_name = runtime.unwrap_or_else(|| "llama-cpp".to_string());

    match runtime_name.as_str() {
        "llama-cpp" => run_with_llama_cpp(&model_path, preset).await,
        "llama-hf" => run_with_llama_hf(model, &quant_str, preset).await,
        "ollama" => run_with_ollama(model, &quant_str).await,
        _ => anyhow::bail!("Unknown runtime: {}. Use 'llama-cpp', 'llama-hf', or 'ollama'", runtime_name),
    }
}

async fn run_with_llama_cpp(model_path: &std::path::Path, preset: Option<String>) -> Result<()> {
    // Check if llama-cli or llama.cpp exists
    let llama_cmd = if command_exists("llama-cli") {
        "llama-cli"
    } else if command_exists("llama") {
        "llama"
    } else if command_exists("main") {
        "main"
    } else {
        println!("{} llama.cpp not found!", "✗".bright_red());
        println!("\n{} Install it first:", "→".bright_blue());
        println!("  Termux: {}", "pkg install llama-cpp".bright_green());
        println!("  Other: {}", "yuy runtime install llama-cpp".bright_green());
        return Ok(());
    };

    println!(
        "{} Starting llama.cpp interactive mode...",
        "▶".bright_green()
    );
    println!();

    // Configure parameters based on preset
    let (temp, top_p) = match preset.as_deref() {
        Some("creative") => (0.8, 0.9),
        Some("precise") => (0.3, 0.5),
        Some("balanced") | None => (0.6, 0.7),
        _ => (0.6, 0.7),
    };

    // Run llama.cpp
    let status = Command::new(llama_cmd)
        .arg("-m")
        .arg(model_path)
        .arg("--interactive")
        .arg("--temp")
        .arg(temp.to_string())
        .arg("--top-p")
        .arg(top_p.to_string())
        .arg("-c")
        .arg("4096")
        .status()
        .context("Failed to execute llama.cpp")?;

    if !status.success() {
        anyhow::bail!("llama.cpp exited with error");
    }

    Ok(())
}

async fn run_with_ollama(model: &str, quant: &str) -> Result<()> {
    if !command_exists("ollama") {
        println!("{} ollama not found!", "✗".bright_red());
        println!("\n{} Install it first:", "→".bright_blue());
        println!("  Termux: {}", "pkg install ollama".bright_green());
        println!("  Other: {}", "yuy runtime install ollama".bright_green());
        return Ok(());
    }

    // Construct ollama model name: aguitachan3/yuuki-best:f32
    let ollama_model = format!("{}/{}:{}", OLLAMA_ORG, model.to_lowercase(), quant);

    println!(
        "{} Starting Ollama with {}...",
        "▶".bright_green(),
        ollama_model.bright_yellow()
    );
    println!();
    
    let status = Command::new("ollama")
        .arg("run")
        .arg(&ollama_model)
        .status()
        .context("Failed to execute ollama")?;

    if !status.success() {
        println!("\n{} Model not found in Ollama.", "ℹ".bright_blue());
        println!("{} Pull it first: {}", "→".bright_blue(), format!("ollama pull {}", ollama_model).bright_green());
    }

    Ok(())
}

async fn run_with_llama_hf(model: &str, quant: &str, preset: Option<String>) -> Result<()> {
    if !command_exists("llama-cli") {
        println!("{} llama-cli not found!", "✗".bright_red());
        println!("\n{} Install llama.cpp first:", "→".bright_blue());
        println!("  Termux: {}", "pkg install llama-cpp".bright_green());
        return Ok(());
    }

    // HuggingFace format: OpceanAI/Yuuki-best:F32
    let hf_model = format!("{}/{}:{}", HF_ORG, model, quant.to_uppercase());

    println!(
        "{} Running directly from HuggingFace: {}",
        "▶".bright_green(),
        hf_model.bright_yellow()
    );
    println!("{} No download needed - streaming from HF", "ℹ".bright_blue());
    println!();

    // Configure parameters based on preset
    let (temp, top_p) = match preset.as_deref() {
        Some("creative") => (0.8, 0.9),
        Some("precise") => (0.3, 0.5),
        Some("balanced") | None => (0.6, 0.7),
        _ => (0.6, 0.7),
    };

    // Run llama-cli with -hf flag
    let status = Command::new("llama-cli")
        .arg("-hf")
        .arg(&hf_model)
        .arg("--temp")
        .arg(temp.to_string())
        .arg("--top-p")
        .arg(top_p.to_string())
        .arg("-c")
        .arg("4096")
        .status()
        .context("Failed to execute llama-cli with HuggingFace")?;

    if !status.success() {
        anyhow::bail!("llama-cli exited with error");
    }

    Ok(())
}
