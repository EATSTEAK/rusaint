cargo run -p uniffi-bindgen generate \
  ./lib/src/main/jniLibs/arm64-v8a/librusaint_ffi.so \
  --library \
  --language kotlin \
  --no-format \
  --out-dir lib/src/main/kotlin