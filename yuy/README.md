# Yuy - Official Yuuki CLI

```
$$\     $$\                    
\$$\   $$  |                   
 \$$\ $$  /$$\   $$\ $$\   $$\ 
  \$$$$  / $$ |  $$ |$$ |  $$ |
   \$$  /  $$ |  $$ |$$ |  $$ |
    $$ |   $$ |  $$ |$$ |  $$ |
    $$ |   \$$$$$$  |\$$$$$$$ |
    \__|    \______/  \____$$ |
                     $$\   $$ |
                     \$$$$$$  |
                      \______/ 
```

Official CLI for Yuuki - AI model management and inference

## Features

✨ **Download Models** - Get Yuuki models from Hugging Face
🚀 **Run Models** - Execute with llama.cpp or ollama
📋 **Manage Models** - List, info, and remove local models
🔧 **Runtime Management** - Install and check llama.cpp/ollama
🏥 **System Doctor** - Diagnose your setup

## Installation

### From Source

```bash
# Clone the repo
git clone https://github.com/YuuKi-OS/yuy
cd yuy

# Build
cargo build --release

# Install (optional)
cargo install --path .
```

### Termux

```bash
pkg install rust
git clone https://github.com/YuuKi-OS/yuy
cd yuy
cargo build --release
```

## Quick Start

```bash
# Initial setup
yuy setup

# Download a model
yuy download Yuuki-best

# Run the model
yuy run Yuuki-best

# Check system health
yuy doctor
```

## Commands

### Download Models

```bash
# Download with auto-selected quantization
yuy download Yuuki-best

# Download specific quantization
yuy download Yuuki-3.7 --quant q8_0

# Available quantizations: q4_0, q5_k_m, q8_0, f32
```

### Run Models

```bash
# Run with default settings
yuy run Yuuki-best

# Specify runtime
yuy run Yuuki-best --runtime llama-cpp

# Use preset configuration
yuy run Yuuki-best --preset creative  # creative, precise, balanced
```

### List Models

```bash
# List local models
yuy list models

# List available models on Hugging Face
yuy list models --remote
```

### Model Information

```bash
# Show model info
yuy info Yuuki-best

# Show available variants
yuy info Yuuki-best --variants
```

### Remove Models

```bash
yuy remove Yuuki-v0.1
```

### Runtime Management

```bash
# Check installed runtimes
yuy runtime check

# Install a runtime (interactive)
yuy runtime install

# Install specific runtime
yuy runtime install llama-cpp

# List available runtimes
yuy runtime list
```

### System Diagnostics

```bash
yuy doctor
```

## Directory Structure

```
~/.yuuki/
├── models/           # Downloaded models
│   ├── Yuuki-best/
│   ├── Yuuki-3.7/
│   └── Yuuki-v0.1/
└── config.toml       # Configuration
```

## Model Quantizations

| Quantization | Size | Quality | Use Case |
|-------------|------|---------|----------|
| `q4_0` | Smallest | Good | Mobile, Termux |
| `q5_k_m` | Medium | Better | Balanced |
| `q8_0` | Large | Best | Desktop |
| `f32` | Largest | Perfect | Full precision |

## Runtimes

### llama.cpp
- Lightweight and fast
- Best for Termux and low-end devices
- Direct CLI usage

### ollama
- User-friendly
- Server-based
- API access

## Development

```bash
# Run in development
cargo run -- download Yuuki-best

# Run tests
cargo test

# Build release
cargo build --release
```

## Platform Support

- ✅ Termux (Android)
- ✅ Linux
- ✅ macOS
- ✅ Windows

## Resources

- **Models**: https://huggingface.co/OpceanAI
- **Training Code**: https://github.com/YuuKi-OS/yuuki-training
- **Issues**: https://github.com/YuuKi-OS/yuy/issues

## License

MIT

---

Made with 🌸 by the Yuuki team
