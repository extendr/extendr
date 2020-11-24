#!/bin/bash

set -euo pipefail

echo pushing with tags to github
git push --tags

echo publishing extendr-macros
pushd extendr-macros
cargo publish
popd

echo waiting for crates.io to catch up
sleep 20

echo publishing extendr-api
pushd extendr-api
cargo publish
popd

