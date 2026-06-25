# silk

[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/0/badge)](https://bestpractices.coreinfrastructure.org/projects/0)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/ljh-sh/silk/badge)](https://scorecard.dev/viewer/?uri=github.com/ljh-sh/silk)

> 微信 SILK 音频转换器。单个 Rust 二进制，无 Python 依赖。

> WeChat SILK audio converter. Single Rust binary, no Python deps. — [English docs](README.md)

## 这是什么

微信语音消息用 [SILK codec](https://en.wikipedia.org/wiki/Silk_(codec)) 编码（Skype 起源）。本工具：

- **解码** SILK → WAV（其他格式在 v0.4）
- **编码** WAV → SILK（重发微信用）
- **识别** 音频格式（按 magic bytes）
- **批量** 处理整个目录

5 个子命令：`decode` / `encode` / `detect` / `info` / `batch`

## 安装

```bash
# 从源码
cargo install --git https://github.com/ljh-sh/silk

# 或从 release 下载
# https://github.com/ljh-sh/silk/releases
```

## 用法

```bash
# 解码微信语音消息
silk decode voice.silk -o voice.wav

# 识别格式
silk detect voice.silk
# 输出：format=silk (extension: silk)

# 显示元信息
silk info voice.silk
# 输出：SILK (WeChat variant, 24000 Hz, mono), 14.94s, ~13.3 kbps

# 批量转换整个目录
silk batch ./wechat_voices/ -o ./mp3s/ --to wav --pattern "*.silk"

# 把 wav 编码回 silk
silk encode speech.wav -o reply.silk
```

## 示例

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

## 对比同类工具

| | silk（本项目） | geniusnut/silk2wav | alexyangfox/wechat_silk | super1207/a2silk-cli |
|---|---|---|---|---|
| 语言 | Rust | Python | Python | Python |
| 单一静态二进制 | ✅ | ❌ | ❌ | ❌ |
| 解码 | ✅ | ✅ | ✅ | ✅ |
| 编码（转回 silk） | ✅ | ❌ | ❌ | ✅ |
| 批量处理 | ✅ 原生 | ❌ | ❌ | ❌ |
| Magic-byte 格式识别 | ✅ | ❌（只靠扩展名） | ❌ | ❌ |
| 依赖 | 5 个纯 Rust crate | Python + ffmpeg | Python | Python |
| 每次启动 | ~10ms | ~40-60ms | 接近 | 接近 |
| 许可证 | Apache 2.0 | MIT | — | — |
| 状态 | 活跃 | 不活跃 | 不活跃 | 不活跃 |

## 性能基准（M 系列 Mac，13 个真实微信 silk 文件，批处理模式）

| 工具 | 总耗时 | 单文件 | 与 silk 对比 |
|---|---|---|---|
| **silk**（Rust） | 127ms | 9.8ms | **1.0x 基线** |
| Python（pysilk） | 327ms | 25.2ms | 0.39x（silk 快 2.5x） |

**2.5x 已经是相当强的结果。** Rust 二进制的主要优势是**零进程启动**——用 xargs 处理 100 个文件时，Python 每次都要付启动税。SILK 解码本身（走同一个 C SDK）速度接近。

复现命令：
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

## 何时用什么

| 场景 | 推荐工具 |
|---|---|
| 单个文件，快速 CLI | silk |
| 100+ 文件批量 | silk（batch） |
| 需要 MP3 输出 | 现用 a2silk-cli，silk v0.4 之后 |
| Python 项目内嵌 | pysilk 直接调 |
| 不信编译后的二进制 | Python 替代品（仍带 C 代码） |

## 安全

- 5 个纯 Rust 依赖 + 1 个 FFI（silk-codec）— 可审计
- 无网络调用、无 telemetry、无自动更新
- 完整依赖审计见 [SECURITY.md](SECURITY.md)

## 路线图

| 版本 | 状态 | 特性 |
|---------|--------|----------|
| v0.2.x | ✅ | WAV + SILK + 5 subcmd，Apache 2.0，性能基准 |
| v0.3.0 | ✅ | 仓库改名、docs/、GitHub Pages |
| v0.4.0 | 📋 | MP3/Opus/AAC/FLAC（symphonia） |

## 许可证

Apache 2.0 — 见 [LICENSE](LICENSE)
