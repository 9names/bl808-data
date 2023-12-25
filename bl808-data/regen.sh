#!/bin/sh
mkdir -p generated
cargo run --release --bin svd_generator
# Convert the SVD to yaml to make it a little easier for humans to read
svdtools convert generated/BL808.svd generated/BL808.yaml
# Convert it back to SVD again, so our shoddily formatted SVD is a bit nicer
svdtools convert generated/BL808.yaml generated/BL808.svd
# Also make a memory map file for quick comparisons
svdtools mmap generated/BL808.svd > generated/BL808.mmap

# Generate yaml files directly too
cargo run --release --bin yaml_generator
