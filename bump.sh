#!/bin/bash

set -euo pipefail

# perhaps someone with bash skill could verify this string.
new_version=$1

# change both tomls to the new version.
sed -i "s/^version = \".*\"/version = \"${new_version}\"/" extendr-api/Cargo.toml
sed -i "s/^extendr-macros.*/extendr-macros = { path = \"..\/extendr-macros\", version=\"${new_version}\" }/" extendr-api/Cargo.toml
sed -i "s/^version = \".*\"/version = \"${new_version}\"/" extendr-macros/Cargo.toml
#git diff
git add extendr-api/Cargo.toml
git add extendr-macros/Cargo.toml
git commit -m "bump to ${new_version} using bump.sh"
git tag v${new_version}

pushd extendr-macros
cargo publish --dry-run
popd

#pushd extendr-api
#cargo publish --dry-run
#popd

#cargo publish
