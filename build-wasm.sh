
#!/bin/sh

# builds all wasm targets

# cargo install dharitri-sc-meta   #teja789

cargo install --path framework/meta

TARGET_DIR=$PWD/target

sc-meta all build --target-dir-all $TARGET_DIR --path ./contracts
sc-meta all build-dbg --target-dir-all $TARGET_DIR --path ./contracts
sc-meta all abi --target-dir-all $TARGET_DIR --path ./contracts