// encode: WAV/MP3/Opus → SILK

use anyhow::{Result, bail};
use std::path::Path;
use crate::codec::{AudioFormat, decode_to_pcm, encode_from_pcm};

pub fn run(input: &Path, output: Option<&Path>, sample_rate: u32) -> Result<()> {
    if sample_rate != 8000 && sample_rate != 12000 && sample_rate != 16000 && sample_rate != 24000 {
        bail!("SILK only supports 8k/12k/16k/24k sample rates, got {}", sample_rate);
    }

    let fmt = AudioFormat::from_magic(input)
        .unwrap_or_else(|_| AudioFormat::Unknown);
    let input_fmt = if fmt == AudioFormat::Unknown {
        AudioFormat::from_extension(input.extension().and_then(|e| e.to_str()).unwrap_or(""))
    } else {
        fmt
    };

    if input_fmt == AudioFormat::Unknown {
        bail!("cannot detect format of {}", input.display());
    }

    let (samples, src_rate) = decode_to_pcm(input, &input_fmt)?;

    // WeChat SILK is typically 24kHz; resample if needed (TODO: actual resampling)
    // For v0.2.0 we just use the source rate
    let _ = src_rate;

    let out = output.map(|p| p.to_path_buf())
        .unwrap_or_else(|| crate::subcmd::decode::default_output(input, "silk"));

    encode_from_pcm(&samples, sample_rate, &AudioFormat::Silk, &out)?;
    println!("encoded {} → {} ({} Hz, {} samples)",
        input.display(), out.display(), sample_rate, samples.len());
    Ok(())
}
