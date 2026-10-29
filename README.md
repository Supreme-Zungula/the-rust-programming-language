# The Rust Programming Language

A hands-on repository for learning Rust by working through [The Rust Book](https://doc.rust-lang.org/book/) (*The Rust Programming Language* by Steve Klabnik and Carol Nichols).

Each chapter folder contains the code written while following the book, plus the end-of-chapter exercises and projects. Personal takeaways and notes live in [`docs/lessons-and-takes.md`](docs/lessons-and-takes.md).

## Repository layout

| Chapter | Topic | Folder |
| --- | --- | --- |
| 1 | Getting started | `chapter-01 Getting started` |
| 2 | Guessing game | `chapter-02 Guessing game` |
| 3 | Common programming concepts | `chapter-03 Common programming concepts` |
| 4 | Understanding ownership | `chapter-04 Understanding Ownership` |
| 5 | Using structs to structure related data | `chapter-05 Using structs to structure related data` |
| 6 | Enums and pattern matching | `chapter-06 Enums and Pattern Matching` |
| 7 | Packages, crates, and modules | `chapter-07 Packages Crates and Modules` |
| 8 | Common collections | `chapter-08 Common Collections` |
| 9 | Error handling | `chapter-09 Error Handling` |
| 10 | Generic types, traits, and lifetimes | `chapter-10 Generic Types Traits and Lifetimes` |
| 11 | Writing automated tests | `chapter-11 Writing Automated Tests` |
| 12 | Building a command line program (`minigrep`) | `chapter-12 Project-Building a command line program` |
| 13 | Functional language features: iterators and closures | `chapter-13 Functional Language features iterators and closures` |

There is **no root Cargo workspace**. Every `Cargo.toml` inside a chapter is its own standalone package, so you `cd` into the specific project before running Cargo commands.

## Prerequisites

- [rustup](https://rustup.rs/) — installs `rustc`, `cargo`, and the standard toolchain
- A linker / C toolchain for your platform (MSVC on Windows, `build-essential` or `base-devel` on Linux)
- A code editor with Rust support, e.g. VS Code with the `rust-analyzer` extension

## Setup on Windows

### 1. Install Rust

The recommended way is `winget`:

```powershell
winget install --id Rustlang.Rustup -e
```

Alternatively, download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

On Windows, Rust needs the MSVC toolchain. If it is not already installed, `rustup-init` will offer to install it; you can also install it explicitly:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e
```

In the Visual Studio Installer, select **Desktop development with C++** so that `link.exe` and the Windows SDK are available.

### 2. Verify the installation

Open a **new** PowerShell window and run:

```powershell
rustc --version
cargo --version
rustup show
```

You should see `stable-x86_64-pc-windows-msvc` as the default toolchain.

### 3. Common Windows issues

- **`error: linker 'link.exe' not found`** — the MSVC build tools are missing. Install them as shown above and run the command from a fresh shell (or from a "Developer PowerShell for VS" session).
- **`cargo` is not recognized** — close and reopen the terminal so the updated `PATH` is picked up, or run `rustup self update` and re-check.

## Setup on WSL

Install Rust **inside WSL**, not just on Windows. A Windows `rustc` cannot build Linux binaries and vice versa.

### 1. Install WSL (from an elevated PowerShell on Windows)

```powershell
wsl --install -d Ubuntu
```

Restart if prompted, then open the Ubuntu shell.

### 2. Install Rust inside WSL

Ubuntu / Debian:

```bash
sudo apt update
sudo apt install -y build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Arch Linux:

```bash
sudo pacman -S --needed base-devel rustup
rustup default stable
```

### 3. Verify the installation

```bash
rustc --version
cargo --version
rustup show
```

Use a stable toolchain:

```bash
rustup default stable
rustup update
```

### 4. WSL tips

- Keep the repository in the Linux filesystem (e.g. `~/Sandbox/...`), **not** under `/mnt/c/...`. Building on the Windows drive is significantly slower and can cause permission problems in `target/`.
- From Windows, the repo is reachable at `\\wsl.localhost\<DistroName>\root\Sandbox\Rust-Programming\the-rust-programming-language`. Opening that path with VS Code (via the WSL extension) lets you edit with `rust-analyzer` running in WSL.

## Running the projects

Folder names contain spaces, so quote paths:

```bash
# Bash / WSL
cd "chapter-02 Guessing game/guessing_game"
cargo run
```

```powershell
# PowerShell
cd "chapter-02 Guessing game\guessing_game"
cargo run
```

Useful Cargo commands:

| Command | Purpose |
| --- | --- |
| `cargo run` | Build and run the current package |
| `cargo build` | Compile a debug build |
| `cargo build --release` | Compile an optimized build |
| `cargo check` | Type-check without producing a binary |
| `cargo test` | Run the test suite (see `chapter-11`) |
| `cargo fmt` | Format the code |
| `cargo clippy` | Run the linter |
| `cargo doc --open` | Build and open docs for dependencies |

To run a single chapter's project from the repository root without changing directory:

```bash
cargo run --manifest-path "chapter-12 Project-Building a command line program/minigrep/Cargo.toml" -- <args>
```

## Notes and conventions

- Chapters use the book's numbering; subfolders follow the section/topic name (e.g. `ch4-2 References and Borrowing`).
- `target/` directories are build artifacts and should not be committed.
- Study notes, summaries, and "gotchas" are collected in [`docs/lessons-and-takes.md`](docs/lessons-and-takes.md).
- Example programs that expect input (like the guessing game) should be run interactively rather than piped.

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standard library documentation](https://doc.rust-lang.org/std/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [rustup documentation](https://rust-lang.github.io/rustup/)
