#!/bin/sh
cargo build --bin svd_generator --release
mkdir -p generated
rm generated/BL616.svd

cargo run --release --bin svd_generator_bl616 > generated/BL616.svd
# Convert the SVD to yaml to make it a little easier for humans to read
svdtools convert generated/BL616.svd generated/BL616.yaml
# Convert it back to SVD again, so our shoddily formatted SVD is a bit nicer
svdtools convert generated/BL616.yaml generated/BL616.svd
# Also make a memory map file for quick comparisons
svdtools mmap generated/BL616.svd > generated/BL616.mmap