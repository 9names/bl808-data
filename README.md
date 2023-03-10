# bl808-data
Scrape useful peripheral/register data out of bl808 sdk

Also contains an SVD generator, currently used for testing scraped data

Source files used to generate the output are in `bl808-data/sources`

## Using the SVD Generator

Output SVD is in `bl808-data/generated`

Running this tool:
- install rust via rustup.rs
- install svdtools via cargo
- run `regen.sh` from inside `bl808-data`

To debug this tool during development:

Add the registers you want to test against to sources/testing/test_registers.h then run

```system
cargo run --bin test_bin > test.svd
```
