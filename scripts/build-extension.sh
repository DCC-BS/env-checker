#!/bin/bash
set -e

echo "Building Environment Checker Zed Extension..."

cd extension
zed build
echo "Extension built successfully: extension.wasm"