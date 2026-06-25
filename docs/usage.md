---
layout: default
title: Usage
---

# Usage

`silk` has 5 subcommands:

```bash
silk <command> [options]
```

## decode — SILK → WAV

```bash
silk decode <input.silk> [-o <output.wav>] [--to wav|pcm]
```

Default output is `<input>.wav` next to the input. `--to pcm` writes raw 16-bit
little-endian samples (no WAV header).

## encode — WAV → SILK

```bash
silk encode <input.wav> [-o <output.silk>] [--sample-rate 8000|12000|16000|24000]
```

WeChat SILK is typically 24kHz. The encoder uses the C SDK so output is bit-exact
for re-sending via WeChat.

## detect — magic-byte format detection

```bash
silk detect <file>
```

Output: `file: format=<format> (extension: <ext>)`. Recognizes:

- `#!SILK_V3` prefix → SILK (WeChat)
- `RIFF...WAVE` → WAV
- `ID3` or `0xFF 0xEx` → MP3
- `OggS` (+ `Opus` in page 2) → OGG or Opus
- `fLaC` → FLAC
- ADTS sync → AAC

## info — file metadata

```bash
silk info <file>
```

For SILK and WAV, prints sample rate, channel count, sample count, duration, and
bitrate. For other formats, prints format name and file size only.

## batch — process a directory

```bash
silk batch <input_dir> [-o <output_dir>] [--to wav] [--pattern "*.silk"]
```

Default output dir is `<input_dir>/converted/`. Default pattern is `*` (all
files). Non-audio files are silently skipped; already-target-format files are
skipped (e.g., running `batch --to wav` on a directory with `.wav` files is a no-op).

## Examples

Decode one file:

```bash
$ silk decode voice.silk -o voice.wav
decoded voice.silk (24000 Hz, 358560 samples) → voice.wav
```

Inspect:

```bash
$ silk info voice.silk
file: voice.silk
format: SILK (WeChat variant, 24000 Hz, mono)
samples: 358560
duration: 14.94s
size: 24812 bytes
bitrate: ~13.3 kbps
```

Batch convert 13 WeChat voice files in one go:

```bash
$ silk batch ./voices/ -o ./mp3s/ --to wav --pattern "*.silk"
✓ decrypted_01.silk → /tmp/out/decrypted_01.wav
... (12 more)
batch done: 13 converted, 0 errors
```
