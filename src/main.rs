use anyhow::{Context, Result};
use clap::Parser;

/// Bir dosyda bir pattern arayın ve pattern içerec satırları görünteleyin.
#[derive(Parser)]
struct Cli {
    /// Aranacak pattern
    pattern: String,
    /// Aranacak dosya yolu
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
