#!/bin/bash
RUSTFLAGS="--remap-path-prefix $PWD=/pwd --remap-path-prefix $HOME/.cargo=/cargo_home" wasm-pack build --release --target web
wc -c pkg/tetris_bg.wasm