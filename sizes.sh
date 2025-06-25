#!/bin/sh

# bytecode sizes

stat --printf="adder %s\n" contracts/examples/adder/output/adder.wasm
stat --printf="crypto-bubbles %s\n" contracts/examples/crypto-bubbles/output/crypto-bubbles.wasm
stat --printf="crowdfunding-erc20 %s\n" contracts/examples/crowdfunding-erc20/output/crowdfunding-erc20.wasm
stat --printf="crowdfunding-dcdt %s\n" contracts/examples/crowdfunding-dcdt/output/crowdfunding-dcdt.wasm
stat --printf="factorial %s\n" contracts/examples/factorial/output/factorial.wasm
stat --printf="lottery-rewa %s\n" contracts/examples/lottery-rewa/output/lottery-rewa.wasm
stat --printf="lottery-erc20 %s\n" contracts/examples/lottery-erc20/output/lottery-erc20.wasm
stat --printf="lottery-dcdt %s\n" contracts/examples/lottery-dcdt/output/lottery-dcdt.wasm
stat --printf="erc20 %s\n" contracts/examples/simple-erc20/output/simple-erc20.wasm
stat --printf="basic-features %s\n" contracts/feature-tests/basic-features/output/basic-features.wasm
stat --printf="async-alice %s\n" contracts/feature-tests/async/async-alice/output/async-alice.wasm
stat --printf="async-bob %s\n" contracts/feature-tests/async/async-bob/output/async-bob.wasm
stat --printf="use-module %s\n" contracts/feature-tests/use-module/output/use-module.wasm
