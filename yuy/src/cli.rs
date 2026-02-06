use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yuy")]
#[command(about = "Official CLI for Yuuki - AI model management and inference")]
#[command(version = "0.1.0")]
#[command(arg_required_else_help = false)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download a model from Hugging Face
    Download {
        /// Model name (Yuuki-best, Yuuki-3.7, Yuuki-v0.1)
        model: String,

        /// Specific quantization to download (q4_0, q5_k_m, q8_0, f32)
        #[arg(short, long)]
        quant: Option<String>,
    },

    /// Run a model with Yuuki Runtime
    Run {
        /// Model name
        model: String,

        /// Runtime to use (llama-cpp or ollama)
        #[arg(short, long)]
        runtime: Option<String>,

        /// Quantization to use
        #[arg(short, long)]
        quant: Option<String>,

        /// Preset configuration (creative, precise, balanced)
        #[arg(short, long)]
        preset: Option<String>,

        /// Resume last conversation
        #[arg(long)]
        resume: bool,

        /// Use a specific template
        #[arg(short, long)]
        template: Option<String>,
    },

    /// List models or other resources
    List {
        #[command(subcommand)]
        target: ListTarget,
    },

    /// Show information about a model
    Info {
        /// Model name
        model: String,

        /// Show available variants/quantizations
        #[arg(long)]
        variants: bool,
    },

    /// Remove a local model
    Remove {
        /// Model name to remove
        model: String,
    },

    /// Manage runtimes (llama.cpp, ollama)
    Runtime {
        #[command(subcommand)]
        action: RuntimeAction,
    },

    /// Check system health and show diagnostics
    Doctor,

    /// Initial setup wizard
    Setup,
}

#[derive(Subcommand)]
pub enum ListTarget {
    /// List local models
    Models {
        /// Show remote models available on Hugging Face
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand)]
pub enum RuntimeAction {
    /// Check installed runtimes
    Check,

    /// Install a runtime
    Install {
        /// Specific runtime to install (llama-cpp or ollama)
        runtime: Option<String>,
    },

    /// List available runtimes
    List,
}
