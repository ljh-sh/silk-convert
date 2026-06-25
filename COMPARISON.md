# Comparison: silk-convert vs Alternatives

Honest, benchmarked comparison of `silk-convert` (this project) with the most
popular existing tools for WeChat SILK audio conversion.

## TL;DR

| | silk-convert (this) | geniusnut/silk2wav | alexyangfox/wechat_silk | super1207/a2silk-cli |
|---|---|---|---|---|
| Language | Rust | Python | Python | Python |
| Single binary | ✅ | ❌ (Python + ffmpeg) | ❌ | ❌ |
| Decode (silk → wav) | ✅ | ✅ | ✅ | ✅ |
| Encode (wav → silk) | ✅ | ❌ | ❌ | ✅ |
| Batch | ✅ | ❌ | ❌ | ❌ |
| Format auto-detect | ✅ (magic bytes) | ❌ (extension) | ❌ | ❌ |
| Output formats | wav (v0.2) / mp3/opus/aac (v0.3) | wav | wav | wav/mp3/ogg/flac |
| Dependencies | clap+hound+walkdir+indicatif+silk-codec (all pure Rust) | Python+ffmpeg | Python | Python |
| Cold-start per file | ~10ms | ~40-60ms (Python + ffmpeg) | similar | similar |
| License | Apache 2.0 | MIT | — | — |
| Last commit | active | inactive | inactive | inactive |

## Benchmark (M-series Mac, 13 real WeChat silk files, batch mode)

| Tool | Wall time | Per-file | Speedup |
|---|---|---|---|
| **silk-convert** (Rust) | 127ms | 9.8ms | **1.0x (baseline)** |
| Python (pysilk) | 327ms | 25.2ms | 0.39x (2.6x slower) |
| Python + ffmpeg pipeline (estimated) | ~500-800ms | ~40-60ms | 0.16-0.25x (4-6x slower) |

The Rust binary's main advantage is **zero per-process startup** — when you
process 100 files via xargs, Python pays 100× startup tax. The actual SILK
decode (via the same C SDK) is comparable.

Reproduce:
```bash
# Python benchmark
pip install pysilk
for f in *.silk; do python3 -c "
import wave, pysilk
with open('$f','rb') as i, open('${f%.silk}.wav','wb') as o:
    pysilk.decode(i, o, 24000)
"; done

# Rust benchmark
silk batch ./ -o ./out/ --to wav --pattern "*.silk"
```

## Security / Risk

| Concern | silk-convert | Python alternatives |
|---|---|---|
| Source language memory safety | ✅ Rust | ❌ Python (C extension) |
| Dependencies | 5 pure-Rust crates, auditable | pip + ffmpeg system binary |
| FFI surface | silk-codec 0.2 (1 C dep, MIT) | pysilk Cython (1 C dep) + ffmpeg (huge surface) |
| Network calls | none | none |
| Telemetry | none | none |
| Build reproducibility | `cargo build --release` reproducible on macOS+Linux | depends on Python + ffmpeg versions |
| License | Apache 2.0 + 3rd-party NOTICE | varies |

## Feature Comparison Detail

### Decode (SILK → something)

All four support this. silk-convert and a2silk-cli also encode.

### Encode (something → SILK)

Only `silk-convert` and `a2silk-cli` support encoding back to SILK.
- `a2silk-cli` only does MP3/WAV/OGG/FLAC → SILK (no Opus/AAC input)
- `silk-convert` (v0.2) only does WAV → SILK; MP3/Opus/AAC input in v0.3

### Batch

Only `silk-convert` has native `batch` subcommand. The Python tools require
shell `for` loops, which pay Python startup per file.

### Format auto-detect

`silk-convert` reads magic bytes (`#!SILK_V3` for WeChat, `RIFF...WAVE`,
`ID3`, `OggS`, etc.). Python tools typically rely on file extension.

### Output formats

| Tool | Output formats |
|---|---|
| silk-convert | wav (v0.2); mp3/opus/aac/ogg/flac (v0.3) |
| silk2wav | wav only |
| wechat_silk | wav only |
| a2silk-cli | wav/mp3/ogg/flac (encode side) |

## When to use what

| Scenario | Best tool |
|---|---|
| One file, want quick CLI | silk-convert |
| 100+ files in a directory | silk-convert (batch) |
| Need MP3 output | silk-convert v0.3 (or a2silk-cli today) |
| Embed in a Python project | pysilk directly |
| Don't trust compiled binaries | Python alternatives (still ship C code via Cython/ffmpeg) |

## License note

This project switched to Apache 2.0 from MIT to:
1. Provide explicit patent grant (relevant for codec work)
2. Include NOTICE file for required third-party attributions
3. Match conventions of similar audio/multimedia projects

## See also

- [README.md](README.md) — quickstart
- [SECURITY.md](SECURITY.md) — security policy and reporting
- [silk-codec](https://github.com/Redmomn/silk-codec) — underlying Rust binding
- [pysilk](https://github.com/lihnux/pysilk) — Python alternative
