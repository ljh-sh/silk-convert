// decode: SILK/WAV → WAV/PCM

use anyhow::{Result, bail};
use std::path::Path;
use crate::codec::{AudioFormat, decode_to_pcm, encode_from_pcm};

pub fn run(input: &Path, output: Option<&Path>, to: &str) -> Result<()> {
    let fmt = AudioFormat::from_magic(input)
        .unwrap_or_else(|_| AudioFormat::Unknown);
    if fmt == AudioFormat::Unknown {
        // fallback to extension
        if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
            let detected = AudioFormat::from_extension(ext);
            if detected == AudioFormat::Unknown {
                bail!("cannot detect format of {}", input.display());
            }
            return run_inner(input, output, to, &detected);
        }
        bail!("cannot detect format of {}", input.display());
    }
    run_inner(input, output, to, &fmt)
}

fn run_inner(input: &Path, output: Option<&Path>, to: &str, input_fmt: &AudioFormat) -> Result<()> {
    let (samples, sample_rate) = decode_to_pcm(input, input_fmt)?;

    let target = match to.to_lowercase().as_str() {
        "wav" => AudioFormat::Wav,
        "pcm" | "raw" => {
            // raw PCM: just write the f32 little-endian
            let out = output.map(|p| p.to_path_buf())
                .unwrap_or_else(|| default_output(input, "pcm"));
            let bytes: Vec<u8> = samples.iter()
                .flat_map(|s| s.to_le_bytes())
                .collect();
            std::fs::write(&out, bytes)?;
            println!("wrote {} samples to {}", samples.len(), out.display());
            return Ok(());
        }
        _ => bail!("unsupported target format: {}", to),
    };

    let out = output.map(|p| p.to_path_buf())
        .unwrap_or_else(|| default_output(input, target.as_str()));
    encode_from_pcm(&samples, sample_rate, &target, &out)?;
    println!("decoded {} ({} Hz, {} samples) → {}",
        input.display(), sample_rate, samples.len(), out.display());
    Ok(())
}

pub fn default_output(input: &Path, ext: &str) -> std::path::PathBuf {
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    let parent = input.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{}.{}", stem, ext))
}
