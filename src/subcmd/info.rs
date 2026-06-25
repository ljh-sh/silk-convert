// info: show file metadata (sample rate, channels, duration, bitrate)

use anyhow::{Result};
use std::path::Path;
use crate::codec::{AudioFormat, decode_to_pcm};

pub fn run(input: &Path) -> Result<()> {
    let fmt = AudioFormat::from_magic(input)?;
    let file_size = std::fs::metadata(input)?.len();

    match fmt {
        AudioFormat::Wav => {
            let reader = hound::WavReader::open(input)?;
            let spec = reader.spec();
            let samples = reader.samples::<i16>().count();
            let duration = samples as f64 / spec.sample_rate as f64;
            println!("file: {}", input.display());
            println!("format: WAV ({} Hz, {} ch, {} bit, {:?})",
                spec.sample_rate, spec.channels, spec.bits_per_sample, spec.sample_format);
            println!("samples: {}", samples);
            println!("duration: {:.2}s", duration);
            println!("size: {} bytes", file_size);
        }
        AudioFormat::Silk => {
            // decode to count samples
            let (samples, sample_rate) = decode_to_pcm(input, &fmt)?;
            let duration = samples.len() as f64 / sample_rate as f64;
            println!("file: {}", input.display());
            println!("format: SILK (WeChat variant, {} Hz, mono)", sample_rate);
            println!("samples: {}", samples.len());
            println!("duration: {:.2}s", duration);
            println!("size: {} bytes", file_size);
            println!("bitrate: ~{:.1} kbps", (file_size as f64 * 8.0) / (duration * 1000.0));
        }
        _ => {
            println!("file: {}", input.display());
            println!("format: {}", fmt.as_str());
            println!("size: {} bytes", file_size);
            println!("(detailed info not yet supported for this format)");
        }
    }
    Ok(())
}
