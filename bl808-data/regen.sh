#!/bin/sh
cargo build --bin svd_generator --release
mkdir -p generated
target/release/svd_generator > generated/BL808.svd
xmlstarlet fo generated/BL808.svd > generated/BL808_formatted.svd
mv generated/BL808_formatted.svd generated/BL808.svd
