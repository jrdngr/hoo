#!/bin/bash

cd hoo_server/hoo-frontend
npm run build
cd ..
cargo run --release