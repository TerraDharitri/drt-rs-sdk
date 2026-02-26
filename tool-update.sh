#!/bin/bash

# Locally update the VM version

VM_TAG="v1.5.1"

echo "Before:"
drtpy config dump
drtpy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
drtpy config dump

drtpy deps install vmtools --overwrite

# Also update the Rust version

drtpy deps install rust --tag="nightly" --overwrite
