#!/bin/bash

# Build sc-meta if not available
if [ ! -f "target/debug/sc-meta" ]; then
    cargo build -p dharitri-sc-meta
fi

cd contracts/benchmarks/mappers
../../../target/debug/sc-meta test > ../../../tools/extract-benchmarks/bench.log
cd ../../..
cd tools/extract-benchmarks
./extract.py
cd ../..
