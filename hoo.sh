#!/bin/bash

cd hoo_server/hoo-frontend
npm install
npm run build
cd ..
cargo run --release