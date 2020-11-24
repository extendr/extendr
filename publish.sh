#!/bin/bash

set -euo pipefail

git push --tags

pushd extendr-macros
cargo publish
popd

pushd extendr-api
cargo publish
popd

