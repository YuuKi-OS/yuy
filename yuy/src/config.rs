use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hf_token: Option<String>,
    pub default_runtime: Option<String>,
    pub default_quant: Option<String>,
    pub default_model: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hf_token: None,
            default_runtime: Some("llama-cpp".to_string()),
            default_quant: Some("q4_k_m".to_string()),
            default_model: Some("Yuuki-NxG-3B".to_string()),
        }
    }
}

pub fn get_yuuki_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let yuuki_dir = home.join(".yuuki");

    if !yuuki_dir.exists() {
        fs::create_dir_all(&yuuki_dir)?;
    }

    Ok(yuuki_dir)
}

pub fn get_models_dir() -> Result<PathBuf> {
    let yuuki_dir = get_yuuki_dir()?;
    let models_dir = yuuki_dir.join("models");

    if !models_dir.exists() {
        fs::create_dir_all(&models_dir)?;
    }

    Ok(models_dir)
}

pub fn get_config_path() -> Result<PathBuf> {
    let yuuki_dir = get_yuuki_dir()?;
    Ok(yuuki_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let content = toml::to_string_pretty(config)?;
    fs::write(config_path, content)?;
    Ok(())
}

// Modelos oficiales OpceanAI en HuggingFace
pub const YUUKI_MODELS: &[(&str, &str)] = &[
    ("Yuuki-NxG-vl",   "OpceanAI/Yuuki-NxG-vl"),   // 7B vision+text
    ("Yuuki-NxG-3B",   "OpceanAI/Yuuki-NxG"),       // 3B conversacional bilingüe
    ("Yuuki-NxG-Nano", "OpceanAI/Yuuki-NxG-Nano"),  // 81M ligero
];

// Modelos cuantizados GGUF disponibles
pub const YUUKI_QUANTIZED_MODELS: &[(&str, &str)] = &[
    ("Yuuki-NxG-vl", "mradermacher/Yuuki-NxG-vl-GGUF"),
];

// Cuantizaciones disponibles
pub const AVAILABLE_QUANTS: &[&str] = &[
    "q2_k",   // 3.02 GB — mínimo
    "q3_k_m", // 3.81 GB — ligero
    "q4_k_m", // 4.68 GB — recomendado
    "q5_k_m", // 5.44 GB — calidad alta
    "q6_k",   // 6.25 GB — casi full
    "q8_0",   // 8.10 GB — máximo cuantizado
    "f16",    // 15.2 GB — full precision
];

pub const HF_ORG: &str = "OpceanAI";
pub const OLLAMA_ORG: &str = "aguitachan3";
pub const YUUKI_API: &str = "https://huggingface.co/spaces/OpceanAI/Yuuki-api";