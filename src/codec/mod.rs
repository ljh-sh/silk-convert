// Audio format detection and codec dispatch

use anyhow::{Result, bail};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioFormat {
    Silk,
    Wav,
    Mp3,
    Opus,
    Ogg,
    Aac,
    Flac,
    Unknown,
}

impl AudioFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioFormat::Silk => "silk",
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Opus => "opus",
            AudioFormat::Ogg => "ogg",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            AudioFormat::Unknown => "unknown",
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "silk" => AudioFormat::Silk,
            "wav" | "pcm" => AudioFormat::Wav,
            "mp3" => AudioFormat::Mp3,
            "opus" => AudioFormat::Opus,
            "ogg" => AudioFormat::Ogg,
            "aac" | "m4a" => AudioFormat::Aac,
            "flac" => AudioFormat::Flac,
            _ => AudioFormat::Unknown,
        }
    }

    /// Detect format by reading file magic bytes
    pub fn from_magic(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        if bytes.len() < 4 {
            return Ok(AudioFormat::Unknown);
        }

        // WAV: "RIFF" .... "WAVE"
        if &bytes[0..4] == b"RIFF" && bytes.len() >= 12 && &bytes[8..12] == b"WAVE" {
            return Ok(AudioFormat::Wav);
        }

        // MP3: "ID3" or 0xFF 0xFB/0xFA/0xF3/0xF2
        if &bytes[0..3] == b"ID3" || (bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0) {
            return Ok(AudioFormat::Mp3);
        }

        // OGG: "OggS"
        if &bytes[0..4] == b"OggS" {
            // could be Opus or Vorbis
            if bytes.len() >= 28 && &bytes[28..32] == b"Opus" {
                return Ok(AudioFormat::Opus);
            }
            return Ok(AudioFormat::Ogg);
        }

        // FLAC: "fLaC"
        if &bytes[0..4] == b"fLaC" {
            return Ok(AudioFormat::Flac);
        }

        // AAC ADTS: 0xFFF sync
        if bytes[0] == 0xFF && (bytes[1] & 0xF6) == 0xF0 {
            return Ok(AudioFormat::Aac);
        }

        // SILK: WeChat variant starts with 0x02 0x00 0x00 (typically)
        // Real SILK has no fixed magic; rely on file size + extension hint
        // For WeChat: check for #!SILK or specific 10-byte header
        if bytes.len() >= 2 && bytes[0] == 0x02 && bytes[1] == 0x00 {
            return Ok(AudioFormat::Silk);
        }

        Ok(AudioFormat::Unknown)
    }
}

/// Read audio samples (mono PCM f32) from any supported format
pub fn decode_to_pcm(path: &Path, format: &AudioFormat) -> Result<(Vec<f32>, u32)> {
    match format {
        AudioFormat::Wav => decode_wav(path),
        AudioFormat::Silk => decode_silk(path),
        AudioFormat::Mp3 => bail!("MP3 decode requires symphonia feature (planned v0.3)"),
        AudioFormat::Opus => bail!("Opus decode requires symphonia feature (planned v0.3)"),
        _ => bail!("decode not yet supported for {:?}", format),
    }
}

fn decode_wav(path: &Path) -> Result<(Vec<f32>, u32)> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader.samples::<i16>().map(|s| s.unwrap_or(0) as f32 / 32768.0).collect(),
        24 => {
            // 24-bit samples stored as i32
            reader.samples::<i32>()
                .map(|s| s.unwrap_or(0) as f32 / 8_388_608.0)
                .collect()
        }
        32 => {
            if spec.sample_format == hound::SampleFormat::Float {
                reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
            } else {
                reader.samples::<i32>().map(|s| s.unwrap_or(0) as f32 / 2_147_483_648.0).collect()
            }
        }
        _ => bail!("unsupported WAV bit depth: {}", spec.bits_per_sample),
    };
    Ok((samples, sample_rate))
}

fn decode_silk(path: &Path) -> Result<(Vec<f32>, u32)> {
    // WeChat SILK: custom format
    // Use silk-codec crate (or implement decoder)
    // For v0.2.0: read 10-byte header, decode with silk-codec
    let bytes = std::fs::read(path)?;
    if bytes.len() < 10 {
        bail!("file too small to be SILK");
    }
    // Strip WeChat 10-byte header
    let silk_data = &bytes[10..];
    let (samples, sample_rate) = silk_codec::decode(silk_data, 24000)?;
    Ok((samples, sample_rate))
}

/// Encode mono PCM to a target format
pub fn encode_from_pcm(samples: &[f32], sample_rate: u32, format: &AudioFormat, path: &Path) -> Result<()> {
    match format {
        AudioFormat::Wav => encode_wav(samples, sample_rate, path),
        AudioFormat::Silk => encode_silk(samples, sample_rate, path),
        _ => bail!("encode not yet supported for {:?}", format),
    }
}

fn encode_wav(samples: &[f32], sample_rate: u32, path: &Path) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        let int_sample = (clamped * 32767.0) as i16;
        writer.write_sample(int_sample)?;
    }
    writer.finalize()?;
    Ok(())
}

fn encode_silk(samples: &[f32], sample_rate: u32, path: &Path) -> Result<()> {
    let silk_data = silk_codec::encode(samples, sample_rate)?;
    // Prepend WeChat 10-byte header (0x02, 0x00, 0x00, ...)
    let mut header = vec![0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut out = header;
    out.extend_from_slice(&silk_data);
    std::fs::write(path, out)?;
    Ok(())
}
