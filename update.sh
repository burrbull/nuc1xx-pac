#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust -i nuc1xx.svd
mv lib.rs src/
form -i ./src/lib.rs -o ./src
cargo fmt
