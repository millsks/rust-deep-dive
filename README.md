# Rust Deep Dive

This repository is for learning Rust by writing, building, and running projects.  
Use this guide to install the core Rust toolchain and set up VS Code on macOS, Linux, and Windows.

## What you need

- **Rustup** (Rust toolchain installer/manager)
- **Rust compiler (`rustc`)**
- **Cargo** (Rust package manager and build tool)
- **Build tools** for your operating system
- **VS Code** + Rust extensions

After installation, verify:

```bash
rustup --version
rustc --version
cargo --version
```

---

## macOS setup (primary)

1. Install Apple command line tools (compiler/linker support):

   ```bash
   xcode-select --install
   ```

2. Install Rust via Rustup:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Restart your terminal, then set the stable toolchain and useful components:

   ```bash
   rustup default stable
   rustup component add rustfmt clippy
   ```

4. (Optional but common) Install Homebrew for additional native dependencies:

   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

---

## Linux setup

1. Install build prerequisites (example for Debian/Ubuntu):

   ```bash
   sudo apt update
   sudo apt install -y build-essential curl pkg-config libssl-dev
   ```

2. Install Rust via Rustup:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Restart your terminal and enable stable + common components:

   ```bash
   rustup default stable
   rustup component add rustfmt clippy
   ```

For Fedora/RHEL-based distros, install equivalent compiler/build packages first (for example `gcc`, `gcc-c++`, `make`, `openssl-devel`, and `pkg-config`).

---

## Windows setup

1. Install **Visual Studio Build Tools** (C++ build tools + Windows SDK).  
   This is required for the default MSVC Rust toolchain.

2. Install Rustup from:

   - https://rustup.rs/

3. Open a new terminal (PowerShell), then run:

   ```powershell
   rustup default stable
   rustup component add rustfmt clippy
   ```

4. Verify:

   ```powershell
   rustup --version
   rustc --version
   cargo --version
   ```

---

## VS Code installation and Rust setup

1. Install VS Code:

   - macOS/Linux/Windows: https://code.visualstudio.com/

2. Install these recommended extensions:

   - **rust-analyzer** (`rust-lang.rust-analyzer`) – IntelliSense, diagnostics, navigation
   - **CodeLLDB** (`vadimcn.vscode-lldb`) – debugging Rust applications
   - **Even Better TOML** (`tamasfe.even-better-toml`) – `Cargo.toml` editing support
   - **crates** (`serayuzgur.crates`) – dependency version hints in `Cargo.toml`

3. Suggested VS Code settings for Rust projects:

   - Enable format on save
   - Use `rustfmt` as formatter
   - Enable Clippy checks while editing

You can also run from terminal in your project:

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo build
cargo test
```
