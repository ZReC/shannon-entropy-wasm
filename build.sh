#!/bin/bash

wasm-pack build --release --no-typescript --target web
cd pkg
rm .gitignore
mv shannon_entropy_wasm.js entropy.js