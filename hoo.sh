#!/bin/bash

cd hoo_frontend
wasm-pack build --target web
cd ..
cargo run --release --bin hoo_server