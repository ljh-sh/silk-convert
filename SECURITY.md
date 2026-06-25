# Security Policy

## Scope

This document describes the security properties of `silk-convert` (a Rust CLI
for converting WeChat SILK audio). It is intended to help users decide whether
to trust the binary.

## What silk-convert does

- Reads audio files (SILK, WAV) from disk
- Decodes/encodes audio in memory or via temporary files
- Writes audio files (WAV, SILK) to disk
- **No network calls. No telemetry. No auto-update.**

## Dependencies

| Crate | Version | License | Surface |
|---|---|---|---|
| clap | 4.5 | MIT OR Apache-2.0 | CLI parsing only |
| anyhow | 1.0 | MIT OR Apache-2.0 | error types only |
| hound | 3.5 | Apache-2.0 | WAV read/write (pure Rust) |
| walkdir | 2.5 | MIT OR Apache-2.0 | directory walking (pure Rust) |
| indicatif | 0.17 | MIT | progress bars (pure Rust) |
| silk-codec | 0.2 | MIT | FFI to SILK SDK (1 C function) |

All 6 dependencies are pure-Rust except `silk-codec` which links the SILK SDK
(a C library, ~3000 LOC, public Skype specification). The C code is sandboxed
to audio processing only.

## FFI surface

The only C code linked is via `silk-codec` 0.2:

- `SKP_Silk_SDK_Get_Decoder_Size` (size query)
- `SKP_Silk_SDK_InitDecoder` (decoder init)
- `SKP_Silk_SDK_Decode` (decode one frame)
- `SKP_Silk_SDK_Encode` (encode one frame)

All input buffers are length-checked by `silk-codec` before passing to the C
SDK. Output buffers are pre-allocated based on SDK-reported max size.

## Build verification

To verify the binary you have matches the source:

```bash
git clone https://github.com/ljh-sh/silk-convert
cd silk-convert
cargo build --release
sha256sum target/release/silk
```

## Reporting a vulnerability

Open a GitHub issue with label `security` (private disclosure preferred via
GitHub Security Advisories).

## What we will NOT do

- We will not add network calls
- We will not add telemetry
- We will not auto-update
- We will not load remote code
