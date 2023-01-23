#!/bin/sh
cargo build --bin svd_generator --release
mkdir -p generated
target/release/svd_generator > generated/BL808.svd
# Convert the SVD to yaml to make it a little easier for humans to read
svdtools convert generated/BL808.svd generated/BL808.yaml
# Convert it back to SVD again, so our shoddily formatted SVD is a bit nicer
svdtools convert generated/BL808.yaml generated/BL808.svd
