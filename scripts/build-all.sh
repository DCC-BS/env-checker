#!/bin/bash
set -e

VERSION=${1:-"0.1.0"}
echo "Building Environment Checker v${VERSION}..."

# Build extension
echo "=== Building Extension ==="
./scripts/build-extension.sh

# Build LSP binaries
echo ""
echo "=== Building LSP Binaries ==="
./scripts/build-lsp-all.sh "$VERSION"

echo ""
echo "=== Build Complete ==="
echo "Extension: extension/extension.wasm"
echo "LSP binaries:"
ls -lh release/*.tar.gz release/*.zip 2>/dev/null || echo "No LSP binaries found"

echo ""
echo "To create a GitHub release, upload:"
echo "  - extension/extension.wasm"
echo "  - All .tar.gz and .zip files from release/"