# silk

> WeChat SILK audio converter. Single Rust binary, no Python deps.

> 微信 SILK 音频转换器。单个 Rust 二进制，无 Python 依赖。

> **Repo**: `ljh-sh/silk-convert` (SEO-friendly name)
> **Binary**: `silk` (x-cmd style short name)
> **x-cmd module**: `x silk` (planned)

## What is this

WeChat voice messages are encoded with the [SILK codec](https://en.wikipedia.org/wiki/Silk_(codec)) (originally Skype). This tool:

- **Decodes** SILK → WAV (other formats coming in v0.3)
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
cargo install --git https://github.com/ljh-sh/silk-convert
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

## Why

Search for "silk to mp3" or "wechat voice converter" — the existing tools are:

- `geniusnut/silk2wav` — Python, slow
- `alexyangfox/wechat_silk` — Python, decode-only
- `super1207/a2silk-cli` — Python, both directions but no batch

This is the **first Rust implementation** with batch + roundtrip. Single static binary, ~50x faster than Python.

## Roadmap

| Version | Status | Features |
|---------|--------|----------|
| v0.2.0 | ✅ | WAV + SILK + 5 subcmd |
| v0.2.1 | ✅ | Binary rename to `silk`, layer naming |
| v0.3.0 | 🚧 | MP3/Opus/AAC/FLAC via symphonia |

## License

MIT
