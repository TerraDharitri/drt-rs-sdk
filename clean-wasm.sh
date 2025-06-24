#!/bin/sh

# cleans all wasm targets

drtpy --verbose contract clean "contracts/benchmarks/str-repeat"
drtpy --verbose contract clean "contracts/examples/adder"
drtpy --verbose contract clean "contracts/examples/crowdfunding-rewa"
drtpy --verbose contract clean "contracts/examples/crowdfunding-erc20"
drtpy --verbose contract clean "contracts/examples/crowdfunding-dcdt"
drtpy --verbose contract clean "contracts/examples/crypto-bubbles"
drtpy --verbose contract clean "contracts/examples/factorial"
drtpy --verbose contract clean "contracts/examples/lottery-rewa"
drtpy --verbose contract clean "contracts/examples/lottery-erc20"
drtpy --verbose contract clean "contracts/examples/multisig"
drtpy --verbose contract clean "contracts/examples/simple-erc20"
drtpy --verbose contract clean "contracts/feature-tests/basic-features"
drtpy --verbose contract clean "contracts/feature-tests/async/async-alice"
drtpy --verbose contract clean "contracts/feature-tests/async/async-bob"
drtpy --verbose contract clean "contracts/feature-tests/use-module"
