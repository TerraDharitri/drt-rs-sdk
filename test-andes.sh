#!/bin/sh

export PATH=$HOME/numbatsdk/andestools:$PATH
cargo test --features numbat-wasm-debug/andes-tests

