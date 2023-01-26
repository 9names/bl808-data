@REM Converts data from Boufallo SDK C headers into SVD format

@REM we use svdtools after generation, ensure it's available
cargo install svdtools
@REM remove any previously generated SVDs and make a dir for them to live in
rmdir /q /s generated
mkdir generated
@REM Build and run the svd_generator
cargo run --release --bin svd_generator > generated/BL808.svd
@REM Convert the SVD to yaml to make it a little easier for humans to read
svdtools convert generated/BL808.svd generated/BL808.yaml
@REM Convert it back to SVD again, so our shoddily formatted SVD is a bit nicer
svdtools convert generated/BL808.yaml generated/BL808.svd
@REM Also make a memory map file for quick comparisons
svdtools mmap generated/BL808.svd > generated/BL808.mmap