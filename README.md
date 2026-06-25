# silk

[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/0/badge)](https://bestpractices.coreinfrastructure.org/projects/0)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/ljh-sh/silk/badge)](https://scorecard.dev/viewer/?uri=github.com/ljh-sh/silk)

> WeChat SILK audio converter. Single Rust binary, no Python deps.

> 微信 SILK 音频转换器。单个 Rust 二进制，无 Python 依赖。

## What is this

WeChat voice messages are encoded with the [SILK codec](https://en.wikipedia.org/wiki/Silk_(codec)) (originally Skype). This tool:

- **Decodes** SILK → WAV (other formats coming in v0.4)
- **Encodes** WAV → SILK (for re-sending via WeChat)
- **Detects** audio format by magic bytes
- **Batch processes** directories

5 subcommands: `decode` / `encode` / `detect` / `info` / `batch`

## 这是什么

微信语音消息用 SILK codec 编码。本工具：

- **解码** SILK → WAV
- **编码** WAV → SILK
- **识别** 音频格式（按 magic bytes）
- **批量** 处理

5 个子命令。

## Install

```bash
# from source
cargo install --git https://github.com/ljh-sh/silk

# or download from releases
# https://github.com/ljh-sh/silk/releases
```

## Usage

```bash
# decode a WeChat voice message
silk decode voice.silk -o voice.wav

# detect format
silk detect voice.silk
# output: format=silk (extension: silk)

# show metadata
silk info voice.silk
# output: SILK (WeChat variant, 24000 Hz, mono), 14.94s, ~13.3 kbps

# batch convert a directory of silk files
silk batch ./wechat_voices/ -o ./mp3s/ --to wav --pattern "*.silk"

# encode wav back to silk
silk encode speech.wav -o reply.silk
```

## Example

```bash
$ silk decode voice.silk -o voice.wav
decoded voice.silk (24000 Hz, 358560 samples) → voice.wav

$ silk info voice.silk
file: voice.silk
format: SILK (WeChat variant, 24000 Hz, mono)
samples: 358560
duration: 14.94s
size: 24812 bytes
bitrate: ~13.3 kbps
```

## Comparison vs alternatives

| | silk (this) | geniusnut/silk2wav | alexyangfox/wechat_silk | super1207/a2silk-cli |
|---|---|---|---|---|
| Language | Rust | Python | Python | Python |
| Single static binary | ✅ | ❌ | ❌ | ❌ |
| Decode | ✅ | ✅ | ✅ | ✅ |
| Encode (back to silk) | ✅ | ❌ | ❌ | ✅ |
| Batch | ✅ native | ❌ | ❌ | ❌ |
| Magic-byte format detect | ✅ | ❌ (extension only) | ❌ | ❌ |
| Dependencies | 6 pure-Rust crates | Python + ffmpeg | Python | Python |
| Cold-start per file | ~10ms | ~40-60ms | similar | similar |
| License | Apache 2.0 | MIT | — | — |
| Status | active | inactive | inactive | inactive |

## Benchmark (M-series Mac, 13 real WeChat silk files, batch mode)

| Tool | Wall time | Per-file | vs silk |
|---|---|---|---|
| **silk** (Rust) | 127ms | 9.8ms | **1.0x baseline** |
| Python (pysilk) | 327ms | 25.2ms | 0.39x (silk 2.5x faster) |

**2.5x faster is already a strong result.** The Rust binary's main advantage
is **zero per-process startup** — when you process 100 files via xargs, Python
pays 100× startup tax. The actual SILK decode (via the same C SDK) is comparable.

Reproduce:
```bash
# Python
pip install pysilk
time for f in *.silk; do python3 -c "
import wave, pysilk
with open('$f','rb') as i, open('${f%.silk}.wav','wb') as o:
    pysilk.decode(i, o, 24000)
"; done

# Rust
time silk batch ./ -o ./out/ --to wav --pattern "*.silk"
```

## When to use what

| Scenario | Best tool |
|---|---|
| One file, quick CLI | silk |
| 100+ files in a directory | silk (batch) |
| Need MP3 output | a2silk-cli today, silk v0.4 later |
| Embed in a Python project | pysilk directly |
| Don't trust compiled binaries | Python alternatives (still ship C code) |

## Security

- 6 pure-Rust deps + 1 FFI (silk-codec) — auditable
- No network calls, no telemetry, no auto-update
- See [SECURITY.md](SECURITY.md) for full dependency audit

## Roadmap

| Version | Status | Features |
|---------|--------|----------|
| v0.2.x | ✅ | WAV + SILK + 5 subcmd, Apache 2.0, benchmark doc |
| v0.3.0 | 🚧 | Repo rename, docs/, GitHub Pages |
| v0.4.0 | 📋 | MP3/Opus/AAC/FLAC via symphonia |

## License

Apache 2.0 — see [LICENSE](LICENSE)
