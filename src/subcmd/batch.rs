// batch: process all silk files in a directory

use anyhow::{Result, bail};
use std::path::Path;
use walkdir::WalkDir;
use crate::codec::{AudioFormat, decode_to_pcm, encode_from_pcm};

pub fn run(input_dir: &Path, output: Option<&Path>, to: &str, pattern: &str) -> Result<()> {
    if !input_dir.is_dir() {
        bail!("{} is not a directory", input_dir.display());
    }

    let out_dir = output.map(|p| p.to_path_buf())
        .unwrap_or_else(|| input_dir.join("converted"));
    std::fs::create_dir_all(&out_dir)?;

    let target = match to.to_lowercase().as_str() {
        "wav" => AudioFormat::Wav,
        _ => bail!("unsupported target format: {} (only wav in v0.2.1)", to),
    };

    let mut count = 0;
    let mut errors = 0;

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();

        if pattern != "*" {
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !simple_glob_match(pattern, fname) {
                continue;
            }
        }

        // detect input format
        let input_fmt = match AudioFormat::from_magic(path) {
            Ok(f) if f != AudioFormat::Unknown => f,
            _ => continue, // skip non-audio files silently
        };

        // skip if input is already the target format
        if input_fmt == target {
            continue;
        }

        let out_path = out_dir.join(
            path.file_stem().and_then(|s| s.to_str()).unwrap_or("out")
        ).with_extension(target.as_str());

        match (|| -> Result<()> {
            let (samples, sr) = decode_to_pcm(path, &input_fmt)?;
            encode_from_pcm(&samples, sr, &target, &out_path)?;
            Ok(())
        })() {
            Ok(_) => {
                println!("✓ {} → {}", path.file_name().and_then(|n| n.to_str()).unwrap_or(""), out_path.display());
                count += 1;
            }
            Err(e) => {
                eprintln!("✗ {} failed: {}", path.display(), e);
                errors += 1;
            }
        }
    }

    println!("\nbatch done: {} converted, {} errors", count, errors);
    if errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn simple_glob_match(pattern: &str, name: &str) -> bool {
    if !pattern.contains('*') {
        return name == pattern;
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 2 {
        name.starts_with(parts[0]) && name.ends_with(parts[1])
    } else {
        false
    }
}
