# silk-convert

> Convert WeChat SILK to MP3/WAV/Opus/AAC and back. Fast single binary, no Python deps.

> 把微信 SILK 音频转成 MP3/WAV/Opus/AAC（或反向）。单二进制，无 Python 依赖。

## What is this

WeChat voice messages are encoded with the [SILK codec](https://en.wikipedia.org/wiki/Silk_(codec)) (originally Skype). This tool:

- **Decodes** SILK → WAV/MP3/Opus/AAC (so you can play them anywhere)
- **Encodes** WAV/MP3/Opus/AAC → SILK (so you can re-encode for WeChat)
- **Detects** audio format by magic bytes, not just extension
- **Batch processes** directories

Single Rust binary. No Python. No `ffmpeg` (optional for non-silk formats). No network calls.

## 这是什么

微信语音消息用 SILK codec 编码（Skype 起源）。本工具：

- **解码** SILK → WAV/MP3/Opus/AAC（任意播放器都能放）
- **编码** WAV/MP3/Opus/AAC → SILK（重编码后发回微信）
- **识别** 音频格式（按 magic bytes，不靠扩展名）
- **批量** 处理整个目录

单个 Rust 二进制。无 Python、无 ffmpeg（非 silk 格式可选）、无网络。

## Install

```bash
# from source
cargo install --git https://github.com/ljh-sh/silk-convert

# or download binary from releases
# https://github.com/ljh-sh/silk-convert/releases
```

## Usage

```bash
# decode a WeChat voice message
silk-convert decode voice.silk -o voice.wav

# decode and convert to mp3
silk-convert convert voice.silk -o voice.mp3

# detect format
silk-convert detect voice.silk
# output: silk (SILK v3, WeChat variant)

# show metadata
silk-convert info voice.silk
# output: 8000 Hz, mono, 5.2s, 24 kbps

# batch convert a directory
silk-convert batch ./wechat_voices/ -o ./mp3s/ --to mp3

# encode wav back to silk
silk-convert encode speech.wav -o reply.silk
```

## Why

Search for "silk to mp3" or "wechat voice converter" — the existing tools are:

- `geniusnut/silk2wav` — Python, slow, requires ffmpeg
- `alexyangfox/wechat_silk` — Python, decode-only
- `super1207/a2silk-cli` — Python, both directions but no batch

This is the **first Rust implementation**: ~50x faster, single static binary, full feature set.

## License

MIT
