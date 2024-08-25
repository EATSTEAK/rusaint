Created with reference to https://github.com/bitwarden/sdk/blob/main/languages/kotlin/README.md

# Android

```bash
cargo install cross --locked --git https://github.com/cross-rs/cross.git --rev 185398b1b885820515a212de720a306b08e2c8c9
```

## building
```bash
mkdir -p ./lib/src/main/jniLibs/{arm64-v8a,x86_64}

cross build -p rusaint-ffi --release --target=aarch64-linux-android
mv ../../target/aarch64-linux-android/release/librusaint_ffi.so ./lib/src/main/jniLibs/arm64-v8a/librusaint_ffi.so

cross build -p rusaint-ffi --release --target=x86_64-linux-android
mv ../../target/x86_64-linux-android/release/librusaint_ffi.so ./lib/src/main/jniLibs/x86_64/librusaint_ffi.so
```

```bash
./build-binding.sh
```