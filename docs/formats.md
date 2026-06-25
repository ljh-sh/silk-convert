---
layout: default
title: Formats
---

# Supported formats

## v0.3.0 (current)

| Format | Decode (input) | Encode (output) |
|---|---|---|
| SILK (WeChat) | ✅ | ✅ |
| WAV (16/24/32-bit, mono/stereo) | ✅ | ✅ |
| PCM raw (16-bit LE) | ✅ (auto) | ✅ (raw mode) |

## v0.4.0 (planned)

| Format | Decode | Encode | Source |
|---|---|---|---|
| MP3 | ✅ | ✅ | symphonia + lame |
| Opus | ✅ | ✅ | symphonia + opus |
| AAC | ✅ | ✅ | symphonia + fdk-aac |
| FLAC | ✅ | ✅ | symphonia |
| OGG Vorbis | ✅ | ✅ | symphonia |

## Detection

Format is detected by magic bytes, not file extension:

| Bytes | Format |
|---|---|
| `#!SILK_V3` (10 bytes, WeChat) | SILK |
| `0x02 0x23 0x21 0x53` (start of `#!SILK_V3`) | SILK |
| `0x02` (first byte, alone) | SILK (guess) |
| `RIFF` ... `WAVE` | WAV |
| `ID3` or `0xFF 0xE?` | MP3 |
| `OggS` | OGG |
| `OggS` + `Opus` (page 2) | Opus |
| `fLaC` | FLAC |
| `0xFF 0xF?` (ADTS sync) | AAC |

When magic detection fails, we fall back to file extension.

## Sample rate

SILK supports 8k, 12k, 16k, 24k sample rates. WeChat voice messages are
typically 24kHz mono. The encoder requires you to specify `--sample-rate` if
your source WAV is at a non-matching rate; in v0.4 we'll add resampling.
