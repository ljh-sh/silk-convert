---
layout: default
title: Home
---

# silk

WeChat voice messages are encoded with the [SILK codec](https://en.wikipedia.org/wiki/Silk_(codec))
(originally developed by Skype). `silk` is a single-binary Rust tool that converts
between SILK and WAV (and back), so you can:

- Play WeChat voice messages in any audio player
- Re-encode audio to send back via WeChat
- Process hundreds of files in a batch

## Why silk?

| | silk | geniusnut/silk2wav | alexyangfox/wechat_silk | super1207/a2silk-cli |
|---|---|---|---|---|
| Language | Rust | Python | Python | Python |
| Single static binary | ✅ | ❌ | ❌ | ❌ |
| Encode (back to silk) | ✅ | ❌ | ❌ | ✅ |
| Batch processing | ✅ native | ❌ | ❌ | ❌ |
| Magic-byte format detect | ✅ | ❌ | ❌ | ❌ |
| Dependencies | 6 pure-Rust crates | Python + ffmpeg | Python | Python |
| Cold-start per file | ~10ms | ~40-60ms | similar | similar |
| License | Apache 2.0 | MIT | — | — |

**2.5x faster** than the Python alternatives, mainly because there's no per-process
Python startup. The actual SILK decode uses the same C SDK underneath.

## Quick start

```bash
# install
cargo install --git https://github.com/ljh-sh/silk

# decode a WeChat voice message
silk decode voice.silk -o voice.wav

# show metadata
silk info voice.silk

# batch convert a directory
silk batch ./wechat_voices/ -o ./mp3s/ --to wav --pattern "*.silk"
```

See the [Usage]({{ "/usage" | relative_url }}) page for more.

## Links

- [GitHub repo](https://github.com/ljh-sh/silk)
- [Releases](https://github.com/ljh-sh/silk/releases)
- [Security policy](https://github.com/ljh-sh/silk/blob/main/SECURITY.md)
