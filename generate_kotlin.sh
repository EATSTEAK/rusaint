#!/bin/bash
cargo run -p uniffi-bindgen generate ./target/release/librusaint_ffi.so --library --language kotlin --out-dir languages/kotlin