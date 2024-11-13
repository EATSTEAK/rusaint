# Generate an xcframework for the Swift bindings.

# Cleanup dirs
rm -r ./Rusaint/Artifacts/RusaintFFI.xcframework
rm -r tmp

mkdir tmp
mkdir -p tmp/target/universal-ios-sim/release

# Build native library
export IPHONEOS_DEPLOYMENT_TARGET="16.0"
export RUSTFLAGS="-C link-arg=-Wl,-application_extension"
cargo build --package rusaint-ffi --target aarch64-apple-ios-sim --release
cargo build --package rusaint-ffi --target aarch64-apple-ios --release
cargo build --package rusaint-ffi --target x86_64-apple-ios --release

# Create universal libraries
lipo -create ../../target/aarch64-apple-ios-sim/release/librusaint_ffi.a \
  ../../target/x86_64-apple-ios/release/librusaint_ffi.a \
  -output ./tmp/target/universal-ios-sim/release/librusaint_ffi.a

# Generate swift bindings
cargo run -p uniffi-bindgen generate \
  ../../target/aarch64-apple-ios-sim/release/librusaint_ffi.dylib \
  --library \
  --language swift \
  --no-format \
  --out-dir tmp/bindings

# Move generated swift bindings
mv ./tmp/bindings/*.swift ./Rusaint/Sources/Rusaint/

# Massage the generated files to fit xcframework
mkdir tmp/Headers
mv ./tmp/bindings/*.h ./tmp/Headers/
cat ./tmp/bindings/*.modulemap > ./tmp/Headers/module.modulemap

# Build xcframework
xcodebuild -create-xcframework \
  -library ../../target/aarch64-apple-ios/release/librusaint_ffi.a \
  -headers ./tmp/Headers \
  -library ./tmp/target/universal-ios-sim/release/librusaint_ffi.a \
  -headers ./tmp/Headers \
  -output ./Rusaint/Artifacts/RusaintFFI.xcframework

# Cleanup temporary files
rm -r tmp