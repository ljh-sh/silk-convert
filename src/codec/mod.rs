// Audio format detection and codec dispatch
// v0.2.1: WAV + SILK (via silk-codec crate)

use anyhow::{Result, bail};
use std::path::Path;
use silk_codec::{decode_silk, encode_silk};

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

        // SILK: WeChat header is "#!SILK_V3" (10 bytes) or just starts with 0x02
        if bytes.len() >= 10 && &bytes[0..10] == b"\x02#!SILK_V3" {
            return Ok(AudioFormat::Silk);
        }
        if bytes.len() >= 2 && bytes[0] == 0x02 {
            return Ok(AudioFormat::Silk);
        }

        Ok(AudioFormat::Unknown)
    }
}

/// Read audio samples (mono PCM f32) from any supported format
pub fn decode_to_pcm(path: &Path, format: &AudioFormat) -> Result<(Vec<f32>, u32)> {
    match format {
        AudioFormat::Wav => decode_wav(path),
        AudioFormat::Silk => decode_silk_file(path),
        AudioFormat::Mp3 => bail!("MP3 decode requires v0.3.0 (symphonia)"),
        AudioFormat::Opus => bail!("Opus decode requires v0.3.0 (symphonia)"),
        AudioFormat::Aac => bail!("AAC decode requires v0.3.0 (symphonia)"),
        AudioFormat::Ogg => bail!("OGG decode requires v0.3.0 (symphonia)"),
        AudioFormat::Flac => bail!("FLAC decode requires v0.3.0 (symphonia)"),
        AudioFormat::Unknown => bail!("unknown format"),
    }
}

fn decode_wav(path: &Path) -> Result<(Vec<f32>, u32)> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader.samples::<i16>().map(|s| s.unwrap_or(0) as f32 / 32768.0).collect(),
        24 => reader.samples::<i32>().map(|s| s.unwrap_or(0) as f32 / 8_388_608.0).collect(),
        32 => {
            if spec.sample_format == hound::SampleFormat::Float {
                reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
            } else {
                reader.samples::<i32>().map(|s| s.unwrap_or(0) as f32 / 2_147_483_648.0).collect()
            }
        }
        _ => bail!("unsupported WAV bit depth: {}", spec.bits_per_sample),
    };
    let mono_samples: Vec<f32> = if spec.channels == 2 {
        samples.chunks(2).map(|c| (c[0] + c.get(1).copied().unwrap_or(0.0)) / 2.0).collect()
    } else {
        samples
    };
    Ok((mono_samples, sample_rate))
}

fn decode_silk_file(path: &Path) -> Result<(Vec<f32>, u32)> {
    // WeChat SILK is typically 24kHz
    let sample_rate: i32 = 24000;
    let raw_bytes = decode_silk(std::fs::read(path)?, sample_rate)?;
    // silk-codec returns raw 16-bit PCM little-endian
    let pcm_samples: Vec<f32> = raw_bytes
        .chunks_exact(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0)
        .collect();
    Ok((pcm_samples, sample_rate as u32))
}

/// Encode mono PCM to a target format
pub fn encode_from_pcm(samples: &[f32], sample_rate: u32, format: &AudioFormat, path: &Path) -> Result<()> {
    match format {
        AudioFormat::Wav => encode_wav(samples, sample_rate, path),
        AudioFormat::Silk => encode_silk_file(samples, sample_rate, path),
        AudioFormat::Mp3 => bail!("MP3 encode requires v0.3.0 (lame)"),
        AudioFormat::Opus => bail!("Opus encode requires v0.3.0 (opus)"),
        AudioFormat::Aac => bail!("AAC encode requires v0.3.0 (fdk-aac)"),
        AudioFormat::Ogg => bail!("OGG encode requires v0.3.0 (vorbis)"),
        AudioFormat::Flac => bail!("FLAC encode requires v0.3.0"),
        AudioFormat::Unknown => bail!("unknown format"),
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

fn encode_silk_file(samples: &[f32], sample_rate: u32, path: &Path) -> Result<()> {
    // Convert f32 samples to 16-bit PCM bytes
    let pcm_bytes: Vec<u8> = samples
        .iter()
        .flat_map(|s| {
            let clamped = s.clamp(-1.0, 1.0);
            let int_sample = (clamped * 32767.0) as i16;
            int_sample.to_le_bytes()
        })
        .collect();

    // Encode to SILK (no tencent header - pure SILK)
    let silk_bytes = encode_silk(&pcm_bytes, sample_rate as i32, 24000, true)?;

    std::fs::write(path, silk_bytes)?;
    Ok(())
}
