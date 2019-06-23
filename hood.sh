#!/bin/bash

cd hoo_frontend
npm run build
cd ..
cp -r ./hoo_frontend/dist ./hoo_server/static
cd hoo_server
cargo run 