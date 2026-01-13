# gitkeep

A simple, cross-platform CLI tool written in Rust to manage `.gitkeep` files in your project directories.

## Features

- Create `.gitkeep` files in a specified directory.
- Custom message support for `.gitkeep` content.
- Option to create empty `.gitkeep` files.
- Recursive mode to apply actions to all subdirectories.
- Easy removal of `.gitkeep` files with the `--let-go` flag.
- Cross-platform support (macOS, Linux, Windows).

## Usage

If no parameters are passed, the help text will be displayed.

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

### Prerequisites
- [Rust and Cargo](https://rustup.rs/)

### Build Steps
1. Clone the repository (or navigate to the project directory).
2. Build the release binary:
   ```bash
   cargo build --release
   ```
3. The compiled binary will be located at:
   `./target/release/gitkeep`

## Testing
A Python test script is provided to verify functionality:
```bash
python3 test_gitkeep.py
```
