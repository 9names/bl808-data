#!/bin/sh
curl https://codeload.github.com/bouffalolab/bouffalo_sdk/zip/302e017ea06b4c75963212f7144f8800c05901f1 -o bouffalo_sdk.zip
unzip -q bouffalo_sdk.zip -d bouffalo_temp
mv bouffalo_temp/bouffalo_sdk-*/drivers/soc/ sources/bouffalo_sdk/
find sources/bouffalo_sdk -type f -iname "CMakeLists.txt" -delete
find sources/bouffalo_sdk -type f -iname "*.a" -delete
find sources/bouffalo_sdk -type f -iname "*.c" -delete
find sources/bouffalo_sdk -type f -iname "*.s" -delete
find sources/bouffalo_sdk -type d -empty -delete
rm -r bouffalo_temp bouffalo_sdk.zip
