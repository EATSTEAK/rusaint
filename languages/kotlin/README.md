# Android
rusaint library for Android

## Prerequisites
- Basic build tools for Android(`gradle`... etc.)
- Android NDK
- [Rust](https://www.rust-lang.org/tools/install) toolchain
- Install `rustup` targets: see below for instructions

```bash
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
```

## Building

```bash
# Make sure rust toolchain and android targets are installed
./gradlew build
```