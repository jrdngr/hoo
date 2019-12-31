#!/bin/bash

cd hoo_frontend
wasm-pack build --target web
cd ..
cargo run --bin hoo_server 