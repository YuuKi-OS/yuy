<div align="center">

<br>

<img src="https://img.shields.io/badge/%E2%9C%A6-YUY-000000?style=for-the-badge&labelColor=000000" alt="Yuy" height="50">

<br><br>

# The CLI for the Yuuki Project

**Download, manage, and run Yuuki models locally.**<br>
**From phones to desktops. One command at a time.**

<br>

<a href="#installation"><img src="https://img.shields.io/badge/GET_STARTED-000000?style=for-the-badge" alt="Get Started"></a>
&nbsp;&nbsp;
<a href="https://huggingface.co/OpceanAI/Yuuki-best"><img src="https://img.shields.io/badge/MODELS-000000?style=for-the-badge" alt="Models"></a>
&nbsp;&nbsp;
<a href="https://huggingface.co/spaces/OpceanAI/Yuuki"><img src="https://img.shields.io/badge/LIVE_DEMO-000000?style=for-the-badge" alt="Demo"></a>

<br><br>

[![License](https://img.shields.io/badge/Apache_2.0-222222?style=flat-square&logo=apache&logoColor=white)](LICENSE)
&nbsp;
[![Rust](https://img.shields.io/badge/Rust_1.75+-222222?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
&nbsp;
[![Termux](https://img.shields.io/badge/Termux-222222?style=flat-square&logo=android&logoColor=white)](#platform-support)
&nbsp;
[![Linux](https://img.shields.io/badge/Linux-222222?style=flat-square&logo=linux&logoColor=white)](#platform-support)
&nbsp;
[![macOS](https://img.shields.io/badge/macOS-222222?style=flat-square&logo=apple&logoColor=white)](#platform-support)
&nbsp;
[![Windows](https://img.shields.io/badge/Windows-222222?style=flat-square&logo=windows&logoColor=white)](#platform-support)

<br>

---

<br>

<table>
<tr>
<td width="50%" valign="top">

```bash
yuy setup
yuy download Yuuki-best
yuy run Yuuki-best
```

</td>
<td width="50%" valign="top">

**Three commands. That's all.**<br><br>
Setup your environment, grab a model,<br>
and start generating code.<br><br>
Yuy handles the rest.

</td>
</tr>
</table>

<br>

</div>

---

<br>

<div align="center">

## What is Yuy?

</div>

<br>

**Yuy** is the command-line interface for the [Yuuki project](https://huggingface.co/OpceanAI) -- an LLM trained entirely on a smartphone with zero budget. Yuy provides a complete toolkit for downloading, managing, and running Yuuki models on any local hardware, with first-class support for mobile devices running Termux.

Under the hood, Yuy wraps proven inference engines (**llama.cpp** and **ollama**) and delivers a streamlined experience on top of them. Model discovery, quantization selection, runtime management, and system diagnostics are handled automatically.

<br>

---

<br>

<div align="center">

## Features

</div>

<br>

<table>
<tr>
<td width="50%" valign="top">

<h3>Model Downloads</h3>

Stream models from Hugging Face with real-time progress bars and auto-selected quantization based on your hardware.

<br>

<h3>Local Inference</h3>

Run models using llama.cpp or ollama. Presets for balanced, creative, and precise generation included.

<br>

<h3>Model Management</h3>

List, inspect metadata, view available quantizations, and remove downloaded models with a single command.

<br>

<h3>System Diagnostics</h3>

Full health check: hardware info, runtime detection, dependency status, and configuration validation.

</td>
<td width="50%" valign="top">

<h3>Runtime Management</h3>

Detect, install, and configure inference runtimes. Yuy guides you through setup on any platform.

<br>

<h3>Cross-Platform</h3>

Termux (Android), Linux, macOS, and Windows. One codebase, consistent experience everywhere.

<br>

<h3>Mobile-First</h3>

Optimized defaults for constrained hardware. Memory-aware quantization, conservative I/O, thermal-safe compilation.

<br>

<h3>Zero Configuration</h3>

Smart defaults that work out of the box. Platform detection, RAM-based recommendations, runtime auto-discovery.

</td>
</tr>
</table>

<br>

---

<br>

<div align="center">

## Installation

</div>

<br>

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or later
- An inference runtime: [llama.cpp](https://github.com/ggerganov/llama.cpp) or [ollama](https://ollama.ai) (Yuy can install these for you)

<br>

### From Source

```bash
git clone https://github.com/YuuKi-OS/yuy
cd yuy
cargo build --release
```

The binary will be at `target/release/yuy`. Install system-wide:

```bash
cargo install --path .
```

<br>

### Termux (Android)

```bash
pkg install rust git
git clone https://github.com/YuuKi-OS/yuy
cd yuy
cargo build --release
```

> **Note:** First compilation on Termux takes longer due to ARM CPU constraints. Subsequent builds use cache and are significantly faster.

<br>

### Verify Installation

```bash
yuy --version
yuy doctor
```

<br>

---

<br>

<div align="center">

## Quick Start

</div>

<br>

```bash
# 1. Initial setup -- creates directories, detects hardware, offers runtime install
yuy setup

# 2. Download a model -- auto-selects best quantization for your hardware
yuy download Yuuki-best

# 3. Run the model -- interactive chat session
yuy run Yuuki-best
```

Yuy handles quantization selection, runtime detection, and parameter configuration automatically.

<br>

---

<br>

<div align="center">

## Commands

</div>

<br>

### `yuy download`

Download Yuuki models from Hugging Face.

```bash
yuy download Yuuki-best                  # auto-select quantization
yuy download Yuuki-best --quant q8_0     # specify quantization
yuy download Yuuki-3.7 --quant q4_0      # different model
```

<details>
<summary><strong>How it works internally</strong></summary>
<br>

1. Validates the model name against the known registry
2. Detects platform and available RAM
3. Recommends the best quantization (or uses your override)
4. Constructs the Hugging Face download URL
5. Streams the file with progress bar showing speed and ETA
6. Saves to `~/.yuuki/models/<model-name>/`

Available quantizations: `q4_0` | `q5_k_m` | `q8_0` | `f32`

</details>

<br>

### `yuy run`

Run a downloaded model with an inference runtime.

```bash
yuy run Yuuki-best                        # defaults
yuy run Yuuki-best --runtime llama-cpp    # specify runtime
yuy run Yuuki-best --preset creative      # use a preset
```

**Generation Presets:**

| Preset | Temperature | Top P | Use Case |
|:-------|:------------|:------|:---------|
| `balanced` | 0.6 | 0.7 | General use **(default)** |
| `creative` | 0.8 | 0.9 | Creative writing, exploration |
| `precise` | 0.3 | 0.5 | Factual, deterministic output |

Yuy detects the available runtime automatically. If both are installed, it defaults to llama.cpp.

<br>

### `yuy list`

List models locally or remotely.

```bash
yuy list models              # downloaded models with sizes
yuy list models --remote     # all available on Hugging Face
```

<details>
<summary><strong>Example output</strong></summary>

```
Local Models:
  Yuuki-best     q4_0     2.3 GB
  Yuuki-3.7      q5_k_m   3.1 GB

Total: 5.4 GB
```

</details>

<br>

### `yuy info`

Display detailed model information.

```bash
yuy info Yuuki-best               # show model info
yuy info Yuuki-best --variants    # available quantizations
```

<br>

### `yuy remove`

Remove a downloaded model.

```bash
yuy remove Yuuki-v0.1
```

Calculates the disk space to be freed and asks for confirmation before deletion.

<br>

### `yuy runtime`

Manage inference runtimes.

```bash
yuy runtime check                  # what's installed
yuy runtime install                # interactive selection
yuy runtime install llama-cpp      # specific runtime
yuy runtime list                   # supported runtimes
```

<details>
<summary><strong>Installation methods by platform</strong></summary>
<br>

| Platform | llama.cpp | ollama |
|:---------|:----------|:-------|
| Termux | `pkg install llama-cpp` | `pkg install ollama` |
| macOS | `brew install llama.cpp` | `brew install ollama` |
| Linux | Binary from GitHub Releases | Official installer |
| Windows | Chocolatey or manual download | Official installer |

</details>

<br>

### `yuy doctor`

Run a full system diagnostic.

<details>
<summary><strong>Example output</strong></summary>

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

</details>

<br>

### `yuy setup`

First-time setup wizard. Creates the `~/.yuuki/` directory structure, detects platform and hardware, checks for runtimes, and offers to install one if none are found.

```bash
yuy setup
```

<br>

---

<br>

<div align="center">

## Model Quantizations

</div>

<br>

Quantization reduces model size at the cost of some precision. Yuy automatically recommends the best option for your hardware.

| Quantization | Size | Quality | Recommended For |
|:-------------|:-----|:--------|:----------------|
| `q4_0` | Smallest | Good | Termux, low-RAM devices (< 8 GB) |
| `q5_k_m` | Medium | Better | Desktop with 8--16 GB RAM |
| `q8_0` | Large | Best | Desktop with 16+ GB RAM |
| `f32` | Largest | Full precision | Research and analysis |

**Auto-selection logic:**

```
Termux (any RAM)           q4_0
Linux/macOS (< 8 GB)       q4_k_m
Linux/macOS (< 16 GB)      q5_k_m   (default)
Linux/macOS (16+ GB)        q8_0
```

<br>

---

<br>

<div align="center">

## Inference Runtimes

</div>

<br>

Yuy delegates inference to external engines. Two runtimes are supported.

<table>
<tr>
<td width="50%" valign="top">

<h3>llama.cpp (default)</h3>

Lightweight, portable, highly optimized. Recommended for most users.

- Single binary, no dependencies
- CPU-optimized with SIMD (NEON on ARM, AVX on x86)
- Optional GPU acceleration (CUDA, Metal, Vulkan)
- Low memory footprint
- Ideal for Termux

**How Yuy invokes it:**

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

</td>
<td width="50%" valign="top">

<h3>ollama</h3>

Server-based runtime with user-friendly model management.

- Built-in model management
- REST API for programmatic access
- Can serve multiple models
- Optional web UI

</td>
</tr>
</table>

<br>

---

<br>

<div align="center">

## Configuration

</div>

<br>

### Config File

Location: `~/.yuuki/config.toml`

```toml
[config]
hf_token = ""                    # Optional: for private models
default_runtime = "llama-cpp"    # llama-cpp | ollama
default_quant = "q5_k_m"         # q4_0 | q5_k_m | q8_0 | f32
```

### Priority Order

Settings resolve in this order (highest priority first):

1. **CLI flags** -- `yuy run Yuuki-best --quant q8_0`
2. **Config file** -- `default_quant = "q5_k_m"`
3. **Auto-detection** -- platform and hardware-based defaults

### Directory Structure

```
~/.yuuki/
    config.toml                  # user configuration
    models/                      # downloaded models
        Yuuki-best/
            yuuki-best-q4_0.gguf
            yuuki-best-q5_k_m.gguf
        Yuuki-3.7/
        Yuuki-v0.1/
```

On Termux the base path is `/data/data/com.termux/files/home/.yuuki/`.

<br>

---

<br>

<div align="center">

## Architecture

</div>

<br>

```
                             User
                               |
                               v
  +------------------------------------------------------------+
  |                     Yuy CLI (Rust)                         |
  |                                                            |
  |   CLI Layer            clap + colored                      |
  |       |                argument parsing, UI, validation    |
  |       v                                                    |
  |   Commands Layer       8 async command modules             |
  |       |                download, run, list, info,          |
  |       |                remove, runtime, doctor, setup      |
  |       v                                                    |
  |   Core Services        config.rs + utils.rs                |
  |                        config management, platform         |
  |                        detection, formatting               |
  +------------+----------------------------+------------------+
               |                            |
               v                            v
     +------------------+        +------------------+
     |  External APIs   |        |  Local Storage   |
     |  Hugging Face    |        |  ~/.yuuki/       |
     |  GitHub          |        |  models + config |
     +---------+--------+        +------------------+
               |
               v
     +-------------------------------+
     |     Inference Runtimes        |
     |   llama.cpp   |    ollama     |
     +-------------------------------+
```

<br>

### Source Layout

```
yuy/
    Cargo.toml                # project manifest and dependencies
    README.md
    PROJECT.md                # technical documentation
    src/
        main.rs               # entry point, CLI router, error handling
        cli.rs                # CLI definitions with clap derive macros
        config.rs             # configuration management, paths, constants
        utils.rs              # platform detection, RAM check, formatting
        commands/
            mod.rs            # module declarations
            download.rs       # model download with streaming + progress
            run.rs            # model execution with runtime detection
            list.rs           # local and remote model listing
            info.rs           # model metadata and variant inspection
            remove.rs         # model deletion with confirmation
            runtime.rs        # runtime detection and installation
            doctor.rs         # system diagnostics
            setup.rs          # first-time setup wizard
```

<br>

### Design Patterns

| Pattern | Implementation |
|:--------|:---------------|
| Command pattern | Each command is an isolated async module with an `execute()` entry point |
| Type-safe CLI | `clap` derive macros ensure compile-time validation of arguments |
| Async I/O | Tokio runtime for non-blocking downloads and process management |
| Error propagation | `anyhow::Result` with contextual error messages throughout |

<br>

### Dependencies

| Crate | Purpose |
|:------|:--------|
| `clap` | CLI argument parsing with derive macros |
| `tokio` | Async runtime |
| `reqwest` | HTTP client for downloads |
| `indicatif` | Progress bars |
| `colored` | Terminal color output |
| `serde` + `toml` | Configuration serialization |
| `dirs` | Cross-platform home directory detection |
| `anyhow` | Error handling |
| `futures-util` | Stream utilities for downloads |

<br>

---

<br>

<div align="center">

## Platform Support

</div>

<br>

| Platform | Status | Notes |
|:---------|:-------|:------|
| **Termux (Android)** | Full support | Primary target, fully tested |
| **Linux x86_64** | Full support | Tested on Ubuntu 22.04+ |
| **Linux ARM64** | Full support | Tested on Raspberry Pi |
| **macOS Intel** | Full support | Tested on Big Sur+ |
| **macOS Apple Silicon** | Full support | Metal acceleration via llama.cpp |
| **Windows 10/11** | Partial | Runtime auto-install not yet implemented |

<br>

<details>
<summary><strong>Termux (Android) -- Primary Target</strong></summary>
<br>

Platform optimizations applied automatically:

- Default quantization: `q4_0` (minimum memory footprint)
- Download buffer: 64 KB (conservative for mobile I/O)
- Compilation: single-threaded (`-j 1`) to avoid thermal throttling
- Progress bars: simplified for narrower terminal widths

Platform detection:

```rust
std::env::var("PREFIX")
    .map(|p| p.contains("com.termux"))
    .unwrap_or(false)
```

</details>

<details>
<summary><strong>Linux Desktop</strong></summary>
<br>

- Default quantization: `q5_k_m`
- Parallel compilation
- GPU support via CUDA or ROCm when available

</details>

<details>
<summary><strong>macOS</strong></summary>
<br>

- Metal acceleration for Apple Silicon GPUs
- Homebrew-based runtime installation
- `q8_0` default on machines with 16+ GB RAM

</details>

<details>
<summary><strong>Windows</strong></summary>
<br>

- Path handling with backslashes
- Chocolatey for package management
- CUDA support for NVIDIA GPUs

</details>

<br>

---

<br>

<div align="center">

## Design Decisions

</div>

<br>

<details>
<summary><strong>Why Rust?</strong></summary>
<br>

Performance with zero runtime overhead, memory safety without a garbage collector, a mature async ecosystem through Tokio, straightforward cross-compilation for all target platforms, and Cargo as a unified build and dependency system.

</details>

<details>
<summary><strong>Why wrap llama.cpp instead of building a custom runtime?</strong></summary>
<br>

Pragmatism. llama.cpp has 3+ years of optimization from 500+ contributors. It handles SIMD, GPU acceleration, quantization formats, and thousands of edge cases. Building an equivalent runtime would take years for a single developer. Yuy provides the experience layer; llama.cpp provides the engine.

</details>

<details>
<summary><strong>Why clap for CLI?</strong></summary>
<br>

clap v4 absorbed structopt, has the strongest documentation in the Rust CLI ecosystem, supports colored help text, and provides compile-time validation through derive macros.

</details>

<details>
<summary><strong>Why TOML for configuration?</strong></summary>
<br>

TOML is more readable than JSON, simpler than YAML, and is the standard in the Rust ecosystem (Cargo.toml). First-class serde support makes serialization trivial.

</details>

<details>
<summary><strong>Why async/await?</strong></summary>
<br>

Large model downloads (multi-GB) must not block the UI. Async enables smooth progress bars and sets the foundation for future parallel chunk downloads.

</details>

<br>

---

<br>

<div align="center">

## Performance

</div>

<br>

| Operation | Target | Actual |
|:----------|:-------|:-------|
| CLI startup | < 100 ms | ~50 ms |
| Download 1 GB | < 5 min | 3--4 min (network dependent) |
| Model listing | < 50 ms | ~10 ms |
| Doctor check | < 200 ms | ~150 ms |

```
Binary size (release):  ~8 MB
Rust source files:      15
Lines of code:          ~2,500
Direct dependencies:    11
Clean build time:       ~2 min
```

<br>

---

<br>

<div align="center">

## Security

</div>

<br>

### Current

- **URL validation** -- only downloads from `https://huggingface.co/`
- **No arbitrary code execution** -- Yuy spawns runtimes, never executes model content
- **Scoped file access** -- all operations within `~/.yuuki/`

### Planned (v0.2+)

- SHA256 checksum verification for downloaded models
- System keyring integration for Hugging Face tokens
- File permission enforcement (`0o600` for sensitive files)
- Encrypted token storage on Termux via libsodium

<br>

---

<br>

<div align="center">

## Roadmap

</div>

<br>

### Phase 1 -- MVP (Complete)

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

### Phase 2 -- Core Features (In Progress)

- [ ] Resume interrupted downloads
- [ ] Parallel chunk downloads
- [ ] SHA256 checksum verification
- [ ] Full ollama integration (Modelfile generation)
- [ ] Automated installation on all platforms
- [ ] Unit and integration tests
- [ ] CI/CD with GitHub Actions

### Phase 3 -- Advanced Features (Planned)

- [ ] Persistent conversation sessions
- [ ] Template system for custom prompts
- [ ] Custom user-defined presets
- [ ] llama.cpp library integration (bypass CLI spawning)
- [ ] Training code download command

### Phase 4 -- Ecosystem (Future)

- [ ] Plugin system
- [ ] Optional web UI
- [ ] REST API server mode
- [ ] Auto-updates
- [ ] Community model hub with ratings
- [ ] Fine-tuning helpers

<br>

---

<br>

<div align="center">

## Contributing

</div>

<br>

### Development Setup

```bash
git clone https://github.com/YuuKi-OS/yuy
cd yuy

# install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install dev tools
cargo install cargo-watch cargo-edit

# verify
cargo check
cargo test
cargo fmt -- --check
cargo clippy
```

### Commit Convention

```
<type>(<scope>): <subject>
```

Types: `feat` | `fix` | `docs` | `style` | `refactor` | `test` | `chore`

Example:

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

<br>

---

<br>

<div align="center">

## About the Yuuki Project

</div>

<br>

Yuy exists to serve the [Yuuki project](https://huggingface.co/OpceanAI/Yuuki-best) -- a code-generation LLM being trained entirely on a smartphone (Redmi 12, Snapdragon 685, CPU only) with zero cloud budget.

<table>
<tr>
<td width="50%" valign="top">

**Training Details**

| | |
|:--|:--|
| Base model | GPT-2 (124M parameters) |
| Training type | Continued pre-training |
| Hardware | Snapdragon 685, CPU only |
| Training time | 50+ hours |
| Progress | 2,000 / 37,500 steps (5.3%) |
| Cost | $0.00 |

</td>
<td width="50%" valign="top">

**Quality Scores (Checkpoint 2000)**

| Language | Score |
|:---------|:------|
| Agda | 55 / 100 |
| C | 20 / 100 |
| Assembly | 15 / 100 |
| Python | 8 / 100 |

</td>
</tr>
</table>

A fully native model (trained from scratch, not fine-tuned) is planned for v1.0. A research paper documenting the mobile training methodology is in preparation.

<br>

---

<br>

<div align="center">

## Links

</div>

<br>

<div align="center">

[![Model Weights](https://img.shields.io/badge/Model_Weights-Hugging_Face-ffd21e?style=for-the-badge&logo=huggingface&logoColor=black)](https://huggingface.co/OpceanAI/Yuuki-best)
&nbsp;
[![Live Demo](https://img.shields.io/badge/Live_Demo-Spaces-ffd21e?style=for-the-badge&logo=huggingface&logoColor=black)](https://huggingface.co/spaces/OpceanAI/Yuuki)
&nbsp;
[![Source Code](https://img.shields.io/badge/Source_Code-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuy)

<br>

[![Training Code](https://img.shields.io/badge/Training_Code-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuuki-training)
&nbsp;
[![Original Model](https://img.shields.io/badge/Original_Model-Historical-555555?style=for-the-badge)](https://huggingface.co/OpceanAI/Yuuki)
&nbsp;
[![Issues](https://img.shields.io/badge/Report_Issue-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuy/issues)

</div>

<br>

---

<br>

<div align="center">

## License

</div>

<br>

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

<br>

---

<br>

<div align="center">

**Built with patience, a phone, and zero budget.**

<br>

[![Yuuki Project](https://img.shields.io/badge/Yuuki_Project-2026-000000?style=for-the-badge)](https://huggingface.co/OpceanAI)

<br>

</div>
