# Android

Created with reference to https://github.com/bitwarden/sdk/blob/main/languages/kotlin/README.md

```bash
cargo install cross
cargo install cross --locked --git https://github.com/cross-rs/cross.git --rev 185398b1b885820515a212de720a306b08e2c8c9
```

## building
```bash
mkdir -p ./sdk/src/main/jniLibs/arm64-v8a

cross build -p rusaint-ffi --release --target=aarch64-linux-android
mv ../../target/aarch64-linux-android/release/librusaint_ffi.so ./lib/src/main/jniLibs/arm64-v8a/librusaint_ffi.so
```

```bash
./build-binginds.sh
```