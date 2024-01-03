#!/bin/bash

cargo build --release --target wasm32-unknown-unknown --features to-my-canvas
rm -fr ./pages
wasm-bindgen --no-typescript --target web --out-dir ./pages/ \
--out-name "bevy-flappy-pebble" ./target/wasm32-unknown-unknown/release/bevy-flappy-pebble.wasm
mkdir ./pages/assets
cp ./assets/*.png ./pages/assets
cp ./assets/index.html ./pages
