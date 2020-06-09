@echo off
cls
wasm-pack build --release --target web crate --out-dir ../release