#!/bin/bash
cargo run -p uniffi-bindgen generate ./target/release/librusaint_ffi.so --library --language swift --out-dir languages/swift