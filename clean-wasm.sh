#!/bin/sh

# cleans all wasm targets

cargo install --path framework/meta --force

sc-meta all clean --path ./contracts
