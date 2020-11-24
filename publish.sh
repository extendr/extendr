#!/bin/bash

set -euo pipefail

pushd extendr-macros
cargo publish
popd

pushd extendr-api
cargo publish
popd

