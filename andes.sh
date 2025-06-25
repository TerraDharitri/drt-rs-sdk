#!/bin/sh

# copies wasm & denali files to the Andes test folder
# expects 1 argument: the path to the Andes repo root

ANDES_PATH=$1

# building all contracts takes a lot of time, here are just the ones for Andes:
drtpy --verbose contract build ./contracts/examples/adder || return 1
drtpy --verbose contract build ./contracts/examples/crowdfunding-dcdt || return 1
drtpy --verbose contract build ./contracts/examples/ping-pong-rewa || return 1
drtpy --verbose contract build ./contracts/examples/multisig || return 1
drtpy --verbose contract build ./contracts/examples/rewa-dcdt-swap || return 1
drtpy --verbose contract build ./contracts/examples/erc20 || return 1
drtpy --verbose contract build ./contracts/feature-tests/basic-features || return 1
drtpy --verbose contract build ./contracts/feature-tests/payable-features || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/async-alice || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/async-bob || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/forwarder || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/forwarder-raw || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/recursive-caller || return 1
drtpy --verbose contract build ./contracts/feature-tests/async/vault || return 1

# if you still want to build all:
# ./build-wasm.sh


# copying the files to andes here:
cp contracts/examples/adder/output/adder.wasm \
   $ANDES_PATH/test/adder/output/adder.wasm
cp -R contracts/examples/adder/denali \
   $ANDES_PATH/test/adder

cp contracts/examples/crowdfunding-dcdt/output/crowdfunding-dcdt.wasm \
   $ANDES_PATH/test/crowdfunding-dcdt/output/crowdfunding-dcdt.wasm
cp -R contracts/examples/crowdfunding-dcdt/denali \
   $ANDES_PATH/test/crowdfunding-dcdt

cp contracts/examples/ping-pong-rewa/output/ping-pong-rewa.wasm \
   $ANDES_PATH/test/ping-pong-rewa/output/ping-pong-rewa.wasm
cp -R contracts/examples/ping-pong-rewa/denali \
   $ANDES_PATH/test/ping-pong-rewa

cp contracts/examples/multisig/output/multisig.wasm \
   $ANDES_PATH/test/multisig/output/multisig.wasm
cp -R contracts/examples/multisig/denali \
   $ANDES_PATH/test/multisig
cp -R contracts/examples/multisig/test-contracts \
   $ANDES_PATH/test/multisig

cp -R contracts/examples/rewa-dcdt-swap/output/rewa-dcdt-swap.wasm \
   $ANDES_PATH/test/rewa-dcdt-swap/output/rewa-dcdt-swap.wasm
cp -R contracts/examples/rewa-dcdt-swap/denali \
   $ANDES_PATH/test/rewa-dcdt-swap

cp -R contracts/examples/erc20/output/erc20.wasm \
   $ANDES_PATH/test/erc20-rust/output/erc20.wasm
cp -R contracts/examples/erc20/denali \
   $ANDES_PATH/test/erc20-rust

cp -R contracts/feature-tests/basic-features/output/basic-features.wasm \
   $ANDES_PATH/test/features/basic-features/output/basic-features.wasm
cp -R contracts/feature-tests/basic-features/denali \
   $ANDES_PATH/test/features/basic-features

cp -R contracts/feature-tests/payable-features/output/payable-features.wasm \
   $ANDES_PATH/test/features/payable-features/output/payable-features.wasm
cp -R contracts/feature-tests/payable-features/denali \
   $ANDES_PATH/test/features/payable-features

cp -R contracts/feature-tests/async/async-alice/output/async-alice.wasm \
   $ANDES_PATH/test/features/async/async-alice/output/async-alice.wasm
cp -R contracts/feature-tests/async/async-bob/output/async-bob.wasm \
   $ANDES_PATH/test/features/async/async-bob/output/async-bob.wasm
cp -R contracts/feature-tests/async/forwarder/output/forwarder.wasm \
   $ANDES_PATH/test/features/async/forwarder/output/forwarder.wasm
cp -R contracts/feature-tests/async/forwarder-raw/output/forwarder-raw.wasm \
   $ANDES_PATH/test/features/async/forwarder-raw/output/forwarder-raw.wasm
cp -R contracts/feature-tests/async/recursive-caller/output/recursive-caller.wasm \
   $ANDES_PATH/test/features/async/recursive-caller/output/recursive-caller.wasm
cp -R contracts/feature-tests/async/vault/output/vault.wasm \
   $ANDES_PATH/test/features/async/vault/output/vault.wasm
cp -R contracts/feature-tests/async/denali \
   $ANDES_PATH/test/features/async
