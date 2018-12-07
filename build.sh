#!/bin/bash
rustc +nightly --target wasm32-unknown-unknown -O src/main.rs
wasm-gc main.wasm
