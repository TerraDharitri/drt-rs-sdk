#!/bin/bash

# copies wasm & denali files to the Andes test folder
# expects 1 argument: the path to the Andes repo root

ANDES_PATH=$1

build_and_copy() {
   contract_path=$1
   contract_name=${contract_path##*/}
   andes_contract_path=$2

   drtpy --verbose contract build $contract_path || return 1
   mkdir -p $andes_contract_path/output
   cp -R $contract_path/output/$contract_name.wasm \
      $andes_contract_path/output/$contract_name.wasm
   cp -R $contract_path/denali \
      $andes_contract_path
}

# building all contracts takes a lot of time, only the ones for Andes are built below
# if you still want to build all:
# ./build-wasm.sh

build_and_copy ./contracts/examples/adder $ANDES_PATH/test/adder
build_and_copy ./contracts/examples/crowdfunding-dcdt $ANDES_PATH/test/crowdfunding-dcdt
build_and_copy ./contracts/examples/ping-pong-rewa $ANDES_PATH/test/ping-pong-rewa
build_and_copy ./contracts/examples/multisig $ANDES_PATH/test/multisig
build_and_copy ./contracts/examples/rewa-dcdt-swap $ANDES_PATH/test/rewa-dcdt-swap
build_and_copy ./contracts/examples/erc20 $ANDES_PATH/test/erc20-rust
build_and_copy ./contracts/feature-tests/basic-features $ANDES_PATH/test/features/basic-features
build_and_copy ./contracts/feature-tests/payable-features $ANDES_PATH/test/features/payable-features

build_and_copy_composability() {
   contract=$1
   contract_with_underscores="${contract//-/_}"

   # with managed-ei
   drtpy --verbose contract build ./contracts/feature-tests/composability/$contract || return 1
   cp -R contracts/feature-tests/composability/$contract/output/${contract}.wasm \
      $ANDES_PATH/test/features/composability/$contract/output/${contract}.wasm

   # without managed-ei
   export RUSTFLAGS=${RUSTFLAGS-'-C link-arg=-s'}
   cd contracts/feature-tests/composability/$contract/wasm-no-managed-ei
   cargo build --target=wasm32-unknown-unknown --release
   cd ..
   mkdir -p output
   cp \
      wasm-no-managed-ei/target/wasm32-unknown-unknown/release/${contract_with_underscores}_wasm.wasm \
      output/${contract}-unmanaged.wasm
   cd ../../../..

   cp -R contracts/feature-tests/composability/$contract/output/${contract}-unmanaged.wasm \
      $ANDES_PATH/test/features/composability/$contract/output/${contract}-unmanaged.wasm
}

build_and_copy_composability forwarder
build_and_copy_composability forwarder-raw
build_and_copy_composability proxy-test-first
build_and_copy_composability proxy-test-second
build_and_copy_composability recursive-caller

drtpy --verbose contract build ./contracts/feature-tests/composability/vault || return 1
cp -R contracts/feature-tests/composability/vault/output/vault.wasm \
   $ANDES_PATH/test/features/composability/vault/output/vault.wasm

cp -R contracts/feature-tests/composability/denali \
   $ANDES_PATH/test/features/composability
rm -f $ANDES_PATH/test/features/composability/denali-legacy/*
mmv -c 'contracts/feature-tests/composability/denali/*.scen.json' \
   $ANDES_PATH/test/features/composability/denali-legacy/l_'#1.scen.json'

sed -i 's/forwarder.wasm/forwarder-unmanaged.wasm/g' $ANDES_PATH/test/features/composability/denali-legacy/*
sed -i 's/forwarder-raw.wasm/forwarder-raw-unmanaged.wasm/g' $ANDES_PATH/test/features/composability/denali-legacy/*
sed -i 's/proxy-test-first.wasm/proxy-test-first-unmanaged.wasm/g' $ANDES_PATH/test/features/composability/denali-legacy/*
sed -i 's/proxy-test-second.wasm/proxy-test-second-unmanaged.wasm/g' $ANDES_PATH/test/features/composability/denali-legacy/*
sed -i 's/recursive-caller.wasm/recursive-caller-unmanaged.wasm/g' $ANDES_PATH/test/features/composability/denali-legacy/*
sed -i 's/proxy_test_init.scen.json/l_proxy_test_init.scen.json/g' $ANDES_PATH/test/features/composability/denali-legacy/*
