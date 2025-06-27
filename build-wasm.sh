# #!/bin/sh

# # builds all wasm targets

# cargo install dharitri-sc-meta                   #teja789

# TARGET_DIR=$PWD/target

# sc-meta all build --target-dir-all $TARGET_DIR --path ./contracts



#!/bin/sh

# builds all wasm targets

# cargo install dharitri-sc-meta   #teja789

cargo install --path /home/dugong/Desktop/teja-work/my_works/rs-sdks/drt-rs-sdk-0.52.3/framework/meta

TARGET_DIR=$PWD/target

sc-meta all build --target-dir-all $TARGET_DIR --path ./contracts
sc-meta all build-dbg --target-dir-all $TARGET_DIR --path ./contracts
sc-meta all abi --target-dir-all $TARGET_DIR --path ./contracts