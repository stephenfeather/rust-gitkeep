use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "gitkeep")]
#[command(about = "A tool to manage .gitkeep files", long_about = None)]
struct Cli {
    /// The path to target
    path: Option<PathBuf>,

    /// Custom message for the .gitkeep file
    #[arg(short, long)]
    message: Option<String>,

    /// Create an empty .gitkeep file
    #[arg(short, long)]
    empty: bool,

    /// Remove the .gitkeep file (Let go)
    #[arg(short = 'l', long = "let-go")]
    let_go: bool,

    /// Recursively apply the action to subdirectories
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let cli = Cli::parse();

    // 9. If no parameters are passed, then display a help text
    if cli.path.is_none() {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        return;
    }

    let target_path = cli.path.unwrap();

    // Determine content
    let content = if cli.empty {
        String::new()
    } else {
        cli.message
            .unwrap_or_else(|| "Created by GitKeep".to_string())
    };

    if cli.recursive {
        process_recursive(&target_path, &content, cli.let_go, cli.empty);
    } else {
        process_single(&target_path, &content, cli.let_go, cli.empty);
    }
}

fn process_single(path: &Path, content: &str, let_go: bool, empty: bool) {
    if !path.exists() {
        eprintln!("Error: Path does not exist: {}", path.display());
        return;
    }

    if !path.is_dir() {
        eprintln!("Error: Path is not a directory: {}", path.display());
        return;
    }

    let file_path = path.join(".gitkeep");

    if let_go {
        if file_path.exists() {
            match fs::remove_file(&file_path) {
                Ok(_) => println!("Removed: {}", file_path.display()),
                Err(e) => eprintln!("Failed to remove {}: {}", file_path.display(), e),
            }
        } else {
            // If not recursive, maybe warn? But usually silent if already gone is fine or "not found".
            // println!("File not found: {}", file_path.display());
        }
    } else {
        match create_gitkeep(&file_path, content, empty) {
            Ok(_) => println!("Created: {}", file_path.display()),
            Err(e) => eprintln!("Failed to create {}: {}", file_path.display(), e),
        }
    }
}

fn process_recursive(root: &Path, content: &str, let_go: bool, empty: bool) {
    for entry in WalkDir::new(root)
        .min_depth(0)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            // We ignore .git directory to avoid messing up repo internals if run at root
            if entry.path().components().any(|c| c.as_os_str() == ".git") {
                continue;
            }

            let file_path = entry.path().join(".gitkeep");

            if let_go {
                if file_path.exists() {
                    match fs::remove_file(&file_path) {
                        Ok(_) => println!("Removed: {}", file_path.display()),
                        Err(e) => eprintln!("Failed to remove {}: {}", file_path.display(), e),
                    }
                }
            } else {
                match create_gitkeep(&file_path, content, empty) {
                    Ok(_) => println!("Created: {}", file_path.display()),
                    Err(e) => eprintln!("Failed to create {}: {}", file_path.display(), e),
                }
            }
        }
    }
}

fn create_gitkeep(path: &Path, content: &str, empty: bool) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    if !empty {
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
