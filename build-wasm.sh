#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### BUILD ALL CONTRACTS ###

drtpy --verbose contract build "contracts/benchmarks/str-repeat"
drtpy --verbose contract build "contracts/examples/adder"
drtpy --verbose contract build "contracts/examples/crowdfunding-rewa"
drtpy --verbose contract build "contracts/examples/crowdfunding-erc20"
drtpy --verbose contract build "contracts/examples/crowdfunding-dcdt"
drtpy --verbose contract build "contracts/examples/crypto-bubbles"
drtpy --verbose contract build "contracts/examples/factorial"
drtpy --verbose contract build "contracts/examples/lottery-rewa"
drtpy --verbose contract build "contracts/examples/lottery-erc20"
drtpy --verbose contract build "contracts/examples/lottery-dcdt"
drtpy --verbose contract build "contracts/examples/multisig"
drtpy --verbose contract build "contracts/examples/non-fungible-tokens"
drtpy --verbose contract build "contracts/examples/simple-erc20"
drtpy --verbose contract build "contracts/feature-tests/abi-tester"
drtpy --verbose contract build "contracts/feature-tests/basic-features"
drtpy --verbose contract build "contracts/feature-tests/async/async-alice"
drtpy --verbose contract build "contracts/feature-tests/async/async-bob"
drtpy --verbose contract build "contracts/feature-tests/use-module"

### CREATE ALL ABIs ###

./abi.sh "contracts/examples/adder"
./abi.sh "contracts/examples/crowdfunding-rewa"
./abi.sh "contracts/examples/crowdfunding-erc20"
./abi.sh "contracts/examples/crowdfunding-dcdt"
./abi.sh "contracts/examples/crypto-bubbles"
./abi.sh "contracts/examples/factorial"
./abi.sh "contracts/examples/lottery-rewa"
./abi.sh "contracts/examples/lottery-erc20"
./abi.sh "contracts/examples/lottery-dcdt"
./abi.sh "contracts/examples/multisig"
./abi.sh "contracts/examples/non-fungible-tokens"
./abi.sh "contracts/examples/simple-erc20"
./abi.sh "contracts/feature-tests/abi-tester"
./abi.sh "contracts/feature-tests/basic-features"
./abi.sh "contracts/feature-tests/use-module"
