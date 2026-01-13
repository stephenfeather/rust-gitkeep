# gitkeep

A simple, cross-platform CLI tool to manage `.gitkeep` files in your project directories.

This project offers two implementations of the same tool:
1. **Rust**: The primary implementation. Prioritizes safety, cross-platform ease (via Cargo), and robust argument parsing.
2. **C++**: A lightweight alternative. Prioritizes minimal binary size and zero dependencies.

## Features

- Create `.gitkeep` files in a specified directory.
- Custom message support for `.gitkeep` content.
- Option to create empty `.gitkeep` files.
- Recursive mode to apply actions to all subdirectories.
- Easy removal of `.gitkeep` files with the `--let-go` flag.
- Cross-platform support (macOS, Linux, Windows).

## Comparison

| Feature | Rust | C++ |
|---------|------|-----|
| **Binary Size (macOS)** | ~823 KB (stripped) | ~39 KB (stripped) |
| **Dependencies** | `clap`, `walkdir` | None (Standard Library) |
| **Build Tool** | `cargo` | `c++` / `g++` / `clang++` |

*Note: Sizes are approximate and based on macOS release builds.*

## Usage

Both binaries share the same command-line interface.

### Basic Creation
Creates a `.gitkeep` with the default message "Created by GitKeep":
```bash
gitkeep /path/to/dir
```

### Custom Message
```bash
gitkeep /path/to/dir -m "My custom message"
```

### Empty File
```bash
gitkeep /path/to/dir -e
```

### Recursive Creation
Creates `.gitkeep` files in the target path and all its subdirectories (automatically skips `.git` folders):
```bash
gitkeep /path/to/dir -r
```

### Remove .gitkeep (Let Go)
Removes the `.gitkeep` file at the specified location:
```bash
gitkeep /path/to/dir -l
# or
gitkeep /path/to/dir --let-go
```

### Recursive Removal
```bash
gitkeep /path/to/dir -lr
```

## How to Build

### Rust Version (Recommended)
**Prerequisites:** [Rust and Cargo](https://rustup.rs/)

1. Build the release binary:
   ```bash
   cargo build --release
   ```
2. The binary will be located at:
   `./target/release/gitkeep`

### C++ Version (Minimal)
**Prerequisites:** A C++17 compatible compiler (GCC, Clang, MSVC).

1. Navigate to the project root.
2. Compile using your preferred compiler:
   ```bash
   # GCC / Clang
   c++ -std=c++17 -O3 src/cpp/main.cpp -o gitkeep_cpp

   # Strip debug symbols for minimum size (optional)
   strip gitkeep_cpp
   ```
3. The binary `gitkeep_cpp` will be created in the current directory.

## Testing
A Python test script is provided to verify functionality. It works with the Rust binary by default but can be modified to test the C++ binary.

```bash
python3 test_gitkeep.py
```