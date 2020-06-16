#!/usr/bin/env bash

# Remove this script once the following issue is addressed:
# https://github.com/rustwasm/wasm-pack/issues/313.

set -e

# Check if jq is installed.
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean previous packages.
if [ -d "pkg" ]; then
    rm -rfv pkg
fi

if [ -d "pkg-node" ]; then
    rm -rfv pkg-node
fi

PKG_NAME="pure-evm"

# Build for both targets.
wasm-pack build -t nodejs -d pkg-node --out-name $PKG_NAME
wasm-pack build -t bundler -d pkg --out-name $PKG_NAME

# Merge nodejs & browser packages.
cp "pkg-node/${PKG_NAME}.js" "pkg/${PKG_NAME}_main.js"
if [[ -f "pkg-node/${PKG_NAME}_bg.js" ]]
then sed "s/require[\(]'\.\/${PKG_NAME}/require\('\.\/${PKG_NAME}_main/" "pkg-node/${PKG_NAME}_bg.js" > "pkg/${PKG_NAME}_bg.js"
fi
jq ".files += [\"${PKG_NAME}_bg.js\"]" pkg/package.json \
    | jq ".main = \"${PKG_NAME}_main.js\"" > pkg/temp.json
mv pkg/temp.json pkg/package.json
rm -rf pkg-node
