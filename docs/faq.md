---
layout: default
title: FAQ
---

# FAQ

### Why is the binary called `silk` but the repo is also `silk`?

The CLI binary is named `silk` (x-cmd style: short and memorable).
The repo is also `ljh-sh/silk` to match. The Rust crate is still
`silk-convert` (for cargo install to work without conflict with the
generic `silk` crate name on crates.io).

### Can I use this with my own x-cmd module?

Yes. Once the binary is on your `$PATH`, you can call it directly:

```bash
silk decode voice.silk -o voice.wav
```

For a proper `x silk` module, see [x-cmd](https://x-cmd.com).

### Is this safe? Does it phone home?

No network calls. No telemetry. No auto-update. The only C code linked is
the SILK SDK (4 FFI functions, all bounded). See [SECURITY.md](https://github.com/ljh-sh/silk/blob/main/SECURITY.md).

### Why is it only 2.5x faster than Python pysilk, not 50x?

Because both use the same C SILK SDK under the hood. The actual decode is
similar speed. The Rust binary wins because it has **zero Python interpreter
startup** — when you process 100 files in a loop, Python pays 100× startup
tax. The Rust binary pays once.

### Does it support Linux ARM64 / Raspberry Pi?

Pre-built binaries are not yet shipped for ARM64 Linux. You can build from
source with `cargo build --release` on any platform with Rust 1.74+.

### What's the difference between SILK v3 and the codec in Skype?

They are the same codec, just framed differently. WeChat uses the v3 framing
with a 10-byte `#!SILK_V3` header (and an extra `0x02` byte prepended by some
clients). The decoder handles both.

### Will it handle corrupted SILK files?

The C SDK will return an error. The Rust wrapper surfaces it as a clear
error message. v0.4 will add better diagnostics.

### Can I use it from a Python script?

Yes, just call the binary via subprocess. For a pure-Python alternative,
use [pysilk](https://pypi.org/project/pysilk/) directly.

### How do I report a security vulnerability?

See [SECURITY.md](https://github.com/ljh-sh/silk/blob/main/SECURITY.md#reporting-a-vulnerability).
