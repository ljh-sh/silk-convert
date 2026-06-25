---
layout: default
title: Install
---

# Install

## Pre-built binaries (recommended)

Download the latest release for your platform from
[GitHub releases](https://github.com/ljh-sh/silk/releases):

| Platform | File |
|---|---|
| macOS (Apple Silicon) | `silk-v0.3.0-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `silk-v0.3.0-x86_64-apple-darwin.tar.gz` |
| Linux (x86_64) | `silk-v0.3.0-x86_64-unknown-linux-gnu.tar.gz` |
| Linux (musl) | `silk-v0.3.0-x86_64-unknown-linux-musl.tar.gz` |
| Windows | `silk-v0.3.0-x86_64-pc-windows-msvc.zip` |

Verify checksums:

```bash
sha256sum -c SHA256SUMS
```

Then put the `silk` binary somewhere on your `$PATH` (e.g., `~/.local/bin` or `/usr/local/bin`).

## From source

Requires [Rust 1.74+](https://rustup.rs/).

```bash
cargo install --git https://github.com/ljh-sh/silk
```

The install produces `~/.cargo/bin/silk`.

## x-cmd module

```bash
# planned: x silk
# currently: download binary or use cargo install
```

## Docker

(not yet available — see [issue tracker](https://github.com/ljh-sh/silk/issues))
