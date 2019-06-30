#!/bin/bash

cd hoo_frontend
npm run build
cd ..
cargo run --bin hoo_server 