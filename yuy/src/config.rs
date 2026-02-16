use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hf_token: Option<String>,
    pub default_runtime: Option<String>,
    pub default_quant: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hf_token: None,
            default_runtime: Some("llama-cpp".to_string()),
            default_quant: Some("q5_k_m".to_string()),
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

pub const YUUKI_MODELS: &[&str] = &["Yuuki-best", "Yuuki-3.7", "Yuuki-v0.1"];
pub const HF_ORG: &str = "OpceanAI";
pub const AVAILABLE_QUANTS: &[&str] = &["q4_0", "q4_k_m", "q5_k_m", "q8_0", "f32"];
pub const OLLAMA_ORG: &str = "aguitachan3";
pub const YUUKI_API: &str = "https://huggingface.co/spaces/OpceanAI/Yuuki-api";
