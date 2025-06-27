#!/bin/sh

# cleans all wasm targets

# cargo install dharitri-sc-meta   #teja789

cargo install --path framework/meta

sc-meta all clean --path ./contracts
