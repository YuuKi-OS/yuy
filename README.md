<p align="center">

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

**The official CLI for the Yuuki project.**
Download, manage, and run Yuuki models locally.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Termux%20%7C%20Linux%20%7C%20macOS%20%7C%20Windows-green.svg)](#platform-support)
[![HuggingFace](https://img.shields.io/badge/models-HuggingFace-yellow.svg)](https://huggingface.co/OpceanAI)

</p>

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
  - [download](#download)
  - [run](#run)
  - [list](#list)
  - [info](#info)
  - [remove](#remove)
  - [runtime](#runtime)
  - [doctor](#doctor)
  - [setup](#setup)
- [Model Quantizations](#model-quantizations)
- [Runtimes](#runtimes)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Platform Support](#platform-support)
- [Platform-Specific Optimizations](#platform-specific-optimizations)
- [Design Decisions](#design-decisions)
- [Performance](#performance)
- [Security](#security)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [About Yuuki](#about-yuuki)
- [Links](#links)
- [License](#license)

---

## About

**Yuy** is the official command-line interface for the [Yuuki project](https://huggingface.co/OpceanAI) â€” an LLM trained entirely on a smartphone. Yuy provides a complete toolkit for downloading, managing, and running Yuuki models on local hardware, with first-class support for mobile devices running Termux.

Yuy wraps proven inference engines (llama.cpp, ollama) and provides an opinionated, streamlined experience on top of them. It handles model discovery, quantization selection, runtime management, and system diagnostics so you can go from zero to inference in three commands.

```
yuy setup
yuy download Yuuki-best
yuy run Yuuki-best
```

---

## Features

- **Download models** from Hugging Face with streaming progress bars and auto-selected quantization
- **Run models** locally using llama.cpp or ollama with preset configurations
- **Manage models** â€” list, inspect, and remove local models
- **Runtime management** â€” detect, install, and configure inference runtimes
- **System diagnostics** â€” check hardware, dependencies, and configuration health
- **Cross-platform** â€” Termux (Android), Linux, macOS, and Windows
- **Mobile-first** â€” optimized defaults for constrained hardware
- **Zero configuration** â€” smart defaults that work out of the box

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or later
- An inference runtime: [llama.cpp](https://github.com/ggerganov/llama.cpp) or [ollama](https://ollama.ai) (Yuy can install these for you)

### From Source

```bash
git clone https://github.com/YuuKi-OS/yuy
cd yuy
cargo build --release
```

The binary will be at `target/release/yuy`. Optionally install it system-wide:

```bash
cargo install --path .
```

### Termux (Android)

```bash
pkg install rust git
git clone https://github.com/YuuKi-OS/yuy
cd yuy
cargo build --release
```

> **Note:** First compilation on Termux takes longer due to ARM CPU constraints. Subsequent builds use cache and are significantly faster.

### Verify Installation

```bash
yuy --version
yuy doctor
```

---

## Quick Start

```bash
# 1. Initial setup â€” creates directories, detects hardware, offers runtime install
yuy setup

# 2. Download a model â€” auto-selects best quantization for your hardware
yuy download Yuuki-best

# 3. Run the model â€” interactive chat session
yuy run Yuuki-best
```

That's it. Yuy handles quantization selection, runtime detection, and parameter configuration automatically.

---

## Commands

### download

Download Yuuki models from Hugging Face.

```bash
# Auto-select best quantization for your hardware
yuy download Yuuki-best

# Specify a quantization
yuy download Yuuki-best --quant q8_0

# Download a different model
yuy download Yuuki-3.7 --quant q4_0
```

**What happens:**

1. Validates the model name against the known model registry
2. Detects your platform and available RAM
3. Recommends the best quantization (or uses your override)
4. Constructs the Hugging Face download URL
5. Streams the file with a progress bar showing speed and ETA
6. Saves to `~/.yuuki/models/<model-name>/`

**Available quantizations:** `q4_0`, `q5_k_m`, `q8_0`, `f32`

---

### run

Run a downloaded model with an inference runtime.

```bash
# Run with defaults
yuy run Yuuki-best

# Specify a runtime
yuy run Yuuki-best --runtime llama-cpp

# Use a preset
yuy run Yuuki-best --preset creative
```

**Presets:**

| Preset | Temperature | Top P | Use Case |
|--------|-------------|-------|----------|
| `balanced` | 0.6 | 0.7 | General use (default) |
| `creative` | 0.8 | 0.9 | Creative writing, exploration |
| `precise` | 0.3 | 0.5 | Factual, deterministic output |

Yuy detects the available runtime automatically. If both llama.cpp and ollama are installed, it defaults to llama.cpp (or your configured preference).

**Runtime detection order for llama.cpp:**

```
llama-cli â†’ llama â†’ main
```

---

### list

List models locally or remotely.

```bash
# List downloaded models with sizes
yuy list models

# List all available models on Hugging Face
yuy list models --remote
```

**Example output:**

```
Local Models:
  Yuuki-best     q4_0     2.3 GB
  Yuuki-3.7      q5_k_m   3.1 GB

Total: 5.4 GB
```

---

### info

Display detailed information about a model.

```bash
# Show model info
yuy info Yuuki-best

# Show available variants/quantizations
yuy info Yuuki-best --variants
```

Shows download status, file sizes, available quantizations, and the path on disk.

---

### remove

Remove a downloaded model.

```bash
yuy remove Yuuki-v0.1
```

Calculates the disk space to be freed and asks for confirmation before deletion.

---

### runtime

Manage inference runtimes.

```bash
# Check what's installed
yuy runtime check

# Install a runtime (interactive selection)
yuy runtime install

# Install a specific runtime
yuy runtime install llama-cpp

# List supported runtimes
yuy runtime list
```

**Installation methods by platform:**

| Platform | llama.cpp | ollama |
|----------|-----------|--------|
| Termux | `pkg install llama-cpp` | `pkg install ollama` |
| macOS | `brew install llama.cpp` | `brew install ollama` |
| Linux | Binary from GitHub Releases | Official installer |
| Windows | Chocolatey or manual download | Official installer |

---

### doctor

Run a full system diagnostic.

```bash
yuy doctor
```

**Example output:**

```
System Information:
  Platform: Termux
  OS: linux
  Arch: aarch64
  RAM: ~6 GB
  Recommended quantization: q4_0

Yuuki Configuration:
  Config dir: /data/data/com.termux/files/home/.yuuki
  Models dir: /data/data/com.termux/files/home/.yuuki/models
  Models downloaded: 2
  Total size: 3.7 GB

Runtime Status:
  [ok] llama.cpp installed (v3.1.0)
  [--] ollama not installed

System Dependencies:
  [ok] curl available
  [ok] wget available
  [--] git not found

Health Summary:
  System is ready to use Yuuki!
```

---

### setup

First-time setup wizard.

```bash
yuy setup
```

Creates the `~/.yuuki/` directory structure, detects your platform and hardware, checks for runtimes, and offers to install one if none are found. Run this once after installation.

---

## Model Quantizations

Quantization reduces model size at the cost of some precision. Yuy automatically recommends the best option for your hardware.

| Quantization | Relative Size | Quality | Recommended For |
|-------------|---------------|---------|-----------------|
| `q4_0` | Smallest | Good | Termux, low-RAM devices (<8 GB) |
| `q5_k_m` | Medium | Better | Desktop with 8-16 GB RAM |
| `q8_0` | Large | Best | Desktop with 16+ GB RAM |
| `f32` | Largest | Full precision | Research, analysis |

**Auto-selection logic:**

```
Termux (any RAM)     â†’ q4_0
Linux/macOS (<8 GB)  â†’ q4_k_m
Linux/macOS (<16 GB) â†’ q5_k_m  (default)
Linux/macOS (16+ GB) â†’ q8_0
```

---

## Runtimes

Yuy delegates inference to external engines. It currently supports two runtimes:

### llama.cpp

The default and recommended runtime. Lightweight, portable, and highly optimized.

- Single binary, no dependencies
- CPU-optimized with SIMD (NEON on ARM, AVX on x86)
- Optional GPU acceleration (CUDA, Metal, Vulkan)
- Low memory footprint
- Ideal for Termux

**How Yuy invokes llama.cpp:**

```bash
llama-cli \
  -m ~/.yuuki/models/Yuuki-best/yuuki-best-q4_0.gguf \
  --interactive \
  --temp 0.7 \
  --top-p 0.9 \
  -c 4096 \
  -n -1 \
  --color
```

### ollama

Server-based runtime with a more user-friendly model management system.

- Built-in model management
- REST API for programmatic access
- Can serve multiple models
- Optional web UI

---

## Configuration

### Config File

Location: `~/.yuuki/config.toml`

```toml
[config]
hf_token = ""                    # Optional: for private models
default_runtime = "llama-cpp"    # llama-cpp | ollama
default_quant = "q5_k_m"         # q4_0 | q5_k_m | q8_0 | f32
```

### Priority Order

Settings are resolved in this order (highest priority first):

1. **CLI flags** â€” `yuy run Yuuki-best --quant q8_0`
2. **Config file** â€” `default_quant = "q5_k_m"`
3. **Auto-detection** â€” platform and hardware-based defaults

### Directory Structure

```
~/.yuuki/
â”œâ”€â”€ config.toml              # User configuration
â””â”€â”€ models/                  # Downloaded models
    â”œâ”€â”€ Yuuki-best/
    â”‚   â”œâ”€â”€ yuuki-best-q4_0.gguf
    â”‚   â””â”€â”€ yuuki-best-q5_k_m.gguf
    â”œâ”€â”€ Yuuki-3.7/
    â””â”€â”€ Yuuki-v0.1/
```

On Termux, the base path is `/data/data/com.termux/files/home/.yuuki/`.

---

## Architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                      User                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                        â”‚
                        v
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                  Yuy CLI (Rust)                       â”‚
â”‚                                                       â”‚
â”‚  CLI Layer â”€â”€â”€â”€â”€â”€â”€â”€ clap + colored                    â”‚
â”‚       â”‚              Argument parsing, UI, validation â”‚
â”‚       v                                               â”‚
â”‚  Commands Layer â”€â”€â”€ 8 async command modules           â”‚
â”‚       â”‚              download, run, list, info,       â”‚
â”‚       â”‚              remove, runtime, doctor, setup   â”‚
â”‚       v                                               â”‚
â”‚  Core Services â”€â”€â”€â”€ config.rs + utils.rs              â”‚
â”‚                      Config management, platform      â”‚
â”‚                      detection, formatting            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           â”‚                      â”‚
           v                      v
  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
  â”‚  External APIs   â”‚    â”‚  Local Storage    â”‚
  â”‚  Hugging Face    â”‚    â”‚  ~/.yuuki/        â”‚
  â”‚  GitHub          â”‚    â”‚  Models + Config  â”‚
  â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           â”‚
           v
  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
  â”‚    Inference Runtimes         â”‚
  â”‚  llama.cpp    â”‚    ollama     â”‚
  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### Source Layout

```
yuy/
â”œâ”€â”€ Cargo.toml          # Project manifest and dependencies
â”œâ”€â”€ README.md
â”œâ”€â”€ PROJECT.md          # Technical documentation
â”‚
â””â”€â”€ src/
    â”œâ”€â”€ main.rs         # Entry point, CLI router, error handling
    â”œâ”€â”€ cli.rs          # CLI definitions with clap derive macros
    â”œâ”€â”€ config.rs       # Configuration management, paths, constants
    â”œâ”€â”€ utils.rs        # Platform detection, RAM check, formatting
    â”‚
    â””â”€â”€ commands/
        â”œâ”€â”€ mod.rs      # Module declarations
        â”œâ”€â”€ download.rs # Model download with streaming + progress
        â”œâ”€â”€ run.rs      # Model execution with runtime detection
        â”œâ”€â”€ list.rs     # Local and remote model listing
        â”œâ”€â”€ info.rs     # Model metadata and variant inspection
        â”œâ”€â”€ remove.rs   # Model deletion with confirmation
        â”œâ”€â”€ runtime.rs  # Runtime detection and installation
        â”œâ”€â”€ doctor.rs   # System diagnostics
        â””â”€â”€ setup.rs    # First-time setup wizard
```

### Design Patterns

- **Command pattern** â€” Each command is an isolated async module with an `execute()` entry point
- **Type-safe CLI** â€” `clap` derive macros ensure compile-time validation of arguments
- **Async I/O** â€” Tokio runtime for non-blocking downloads and process management
- **Error propagation** â€” `anyhow::Result` with contextual error messages throughout

### Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `tokio` | Async runtime |
| `reqwest` | HTTP client for downloads |
| `indicatif` | Progress bars |
| `colored` | Terminal color output |
| `serde` + `toml` | Configuration serialization |
| `dirs` | Cross-platform home directory detection |
| `anyhow` | Error handling |
| `futures-util` | Stream utilities for downloads |

---

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Termux (Android) | Full support | Primary target, fully tested |
| Linux (x86_64) | Full support | Tested on Ubuntu 22.04+ |
| Linux (ARM64) | Full support | Tested on Raspberry Pi |
| macOS (Intel) | Full support | Tested on Big Sur+ |
| macOS (Apple Silicon) | Full support | Metal acceleration via llama.cpp |
| Windows 10/11 | Partial | Runtime auto-install not yet implemented |

---

## Platform-Specific Optimizations

### Termux (Android)

Termux is the primary target. Yuy applies these optimizations automatically:

- **Default quantization:** `q4_0` (minimum memory footprint)
- **Download buffer:** 64 KB (conservative for mobile I/O)
- **Compilation:** Single-threaded (`-j 1`) to avoid thermal throttling
- **Progress bars:** Simplified for narrower terminal widths

**Platform detection:**

```rust
std::env::var("PREFIX")
    .map(|p| p.contains("com.termux"))
    .unwrap_or(false)
```

### Linux Desktop

- Default quantization: `q5_k_m`
- Parallel compilation
- GPU support via CUDA or ROCm when available

### macOS

- Metal acceleration for Apple Silicon GPUs
- Homebrew-based runtime installation
- `q8_0` default on machines with 16+ GB RAM

### Windows

- Path handling with backslashes
- Chocolatey for package management
- CUDA support for NVIDIA GPUs

---

## Design Decisions

### Why Rust?

- **Performance** â€” small, fast binaries with no runtime overhead
- **Memory safety** â€” no garbage collector, no segfaults
- **Async ecosystem** â€” Tokio provides mature non-blocking I/O
- **Cross-compilation** â€” single codebase targets all platforms
- **Cargo** â€” dependency management and build system in one tool

### Why wrap llama.cpp instead of building a custom runtime?

Pragmatism. llama.cpp has 3+ years of optimization work from 500+ contributors. It handles SIMD, GPU acceleration, quantization formats, and thousands of edge cases. Building an equivalent would take years for a single developer. Yuy provides the experience layer; llama.cpp provides the engine.

### Why clap for CLI?

clap v4 absorbed structopt, has the best documentation in the Rust CLI ecosystem, supports colored help text, and provides compile-time validation through derive macros.

### Why TOML for configuration?

TOML is more readable than JSON, simpler than YAML, and is the standard in the Rust ecosystem (Cargo.toml). First-class serde support makes serialization trivial.

### Why async/await?

Large model downloads (multi-GB) must not block the UI. Async enables smooth progress bars, and sets the foundation for future parallel chunk downloads.

---

## Performance

### Benchmarks

| Operation | Target | Actual |
|-----------|--------|--------|
| CLI startup | <100 ms | ~50 ms |
| Download 1 GB | <5 min | 3-4 min (network dependent) |
| Model listing | <50 ms | ~10 ms |
| Doctor check | <200 ms | ~150 ms |

### Binary Size

```
Release build: ~8 MB
```

### Code Statistics

```
Rust source files:  15
Lines of code:      ~2,500
Direct dependencies: 11
Clean build time:    ~2 min
```

---

## Security

### Current Measures

- **URL validation** â€” only downloads from `https://huggingface.co/`
- **No arbitrary code execution** â€” Yuy spawns runtimes, never executes model content
- **Scoped file access** â€” all operations within `~/.yuuki/`

### Planned (v0.2+)

- SHA256 checksum verification for downloaded models
- System keyring integration for Hugging Face tokens (instead of plaintext in config)
- File permission enforcement (`0o600` for sensitive files)
- Encrypted token storage on Termux via libsodium

---

## Roadmap

### Phase 1: MVP (Complete)

- [x] Core CLI with 8 commands
- [x] Download from Hugging Face with progress bars
- [x] Run models with llama.cpp
- [x] Model management (list, info, remove)
- [x] Runtime detection and installation
- [x] System diagnostics
- [x] Setup wizard
- [x] Multi-platform support
- [x] Auto-selection of quantization
- [x] Colored terminal output

### Phase 2: Core Features (In Progress)

- [ ] Resume interrupted downloads
- [ ] Parallel chunk downloads
- [ ] SHA256 checksum verification
- [ ] Full ollama integration (Modelfile generation)
- [ ] Automated installation on all platforms
- [ ] Unit and integration tests
- [ ] CI/CD with GitHub Actions

### Phase 3: Advanced Features (Planned)

- [ ] Persistent conversation sessions
  ```
  ~/.yuuki/conversations/
  â”œâ”€â”€ session-2026-01-15.json
  â””â”€â”€ session-2026-01-16.json
  ```
- [ ] Template system for custom prompts
  ```bash
  yuy template create coding-assistant
  yuy run Yuuki-best --template coding-assistant
  ```
- [ ] Custom user-defined presets
  ```toml
  [presets.my-creative]
  temperature = 0.9
  top_p = 0.95
  top_k = 50
  ```
- [ ] llama.cpp library integration (bypass CLI spawning)
- [ ] Training code download command

### Phase 4: Ecosystem (Future)

- [ ] Plugin system
- [ ] Optional web UI
- [ ] REST API server mode
- [ ] Auto-updates
- [ ] Optional telemetry (opt-in)
- [ ] Community model hub with ratings
- [ ] Fine-tuning helpers

---

## Contributing

### Development Setup

```bash
# Clone
git clone https://github.com/YuuKi-OS/yuy
cd yuy

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dev tools
cargo install cargo-watch cargo-edit

# Verify
cargo check
cargo test
cargo fmt -- --check
cargo clippy
```

### Commit Convention

```
<type>(<scope>): <subject>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Example:**

```
feat(download): add resume capability

- Implement Range headers for resume
- Save download state in .partial files
- Auto-recover on failure

Closes #42
```

### Pull Request Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] Commits follow the convention above

### Coding Standards

- `snake_case` for functions, `CamelCase` for types
- Document all public functions
- Use `Result<T>` and the `?` operator for error handling
- Prefer `async/await` over callbacks
- Justify any new dependency

---

## About Yuuki

Yuy exists to serve the [Yuuki project](https://huggingface.co/OpceanAI/Yuuki-best) â€” a code-generation LLM being trained entirely on a smartphone (Redmi 12, Snapdragon 685, CPU only) with zero cloud budget.

**Key facts about the model:**

| Detail | Value |
|--------|-------|
| Base model | GPT-2 (124M parameters) |
| Training type | Continued pre-training (fine-tuning) |
| Hardware | Snapdragon 685, CPU only |
| Training time | 50+ hours |
| Progress | 2,000 / 37,500 steps (5.3%) |
| Cost | $0.00 |
| Best language | Agda (55/100) |
| License | Apache 2.0 |

**Current quality scores (Checkpoint 2000):**

| Language | Score |
|----------|-------|
| Agda | 55/100 |
| C | 20/100 |
| Assembly | 15/100 |
| Python | 8/100 |

A fully native model (trained from scratch, not fine-tuned) is planned for v1.0. A research paper documenting the mobile training methodology is in preparation.

---

## Links

| Resource | URL |
|----------|-----|
| Model weights (recommended) | https://huggingface.co/OpceanAI/Yuuki-best |
| Original model (historical) | https://huggingface.co/OpceanAI/Yuuki |
| Interactive demo | https://huggingface.co/spaces/OpceanAI/Yuuki |
| Training code | https://github.com/YuuKi-OS/yuuki-training |
| CLI source (this repo) | https://github.com/YuuKi-OS/yuy |
| Issues | https://github.com/YuuKi-OS/yuy/issues |

---

## License

Licensed under the **Apache License, Version 2.0**.

```
Copyright 2026 Yuuki Project

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

---

<p align="center">
  <b>Built with patience, a phone, and zero budget.</b><br>
  Yuuki Project
</p>
