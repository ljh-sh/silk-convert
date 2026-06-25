// convert: between any two supported formats (auto-detect input)

use anyhow::{Result, bail};
use std::path::Path;
use crate::codec::{AudioFormat, decode_to_pcm, encode_from_pcm};

pub fn run(input: &Path, output: Option<&Path>, to: &str) -> Result<()> {
    let target = match to.to_lowercase().as_str() {
        "wav" => AudioFormat::Wav,
        "mp3" => AudioFormat::Mp3,
        "opus" => AudioFormat::Opus,
        "aac" | "m4a" => AudioFormat::Aac,
        "silk" => AudioFormat::Silk,
        "ogg" => AudioFormat::Ogg,
        "flac" => AudioFormat::Flac,
        _ => bail!("unsupported target format: {}", to),
    };

    let fmt = AudioFormat::from_magic(input)
        .unwrap_or_else(|_| AudioFormat::Unknown);
    let source = if fmt == AudioFormat::Unknown {
        AudioFormat::from_extension(input.extension().and_then(|e| e.to_str()).unwrap_or(""))
    } else {
        fmt
    };

    if source == AudioFormat::Unknown {
        bail!("cannot detect format of {}", input.display());
    }
    if source == target {
        bail!("source and target are both {:?}", source);
    }

    let (samples, sample_rate) = decode_to_pcm(input, &source)?;
    let out = output.map(|p| p.to_path_buf())
        .unwrap_or_else(|| crate::subcmd::decode::default_output(input, target.as_str()));
    encode_from_pcm(&samples, sample_rate, &target, &out)?;

    println!("converted {} ({}) → {} ({})",
        input.display(), source.as_str(),
        out.display(), target.as_str());
    Ok(())
}
