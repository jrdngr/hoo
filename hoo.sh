#!/bin/bash

cd hoo_frontend
npm run build
cd ..
cargo run --release --bin hoo_server