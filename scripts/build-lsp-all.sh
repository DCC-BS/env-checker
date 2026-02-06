#!/bin/bash
set -e

VERSION=${1:-"0.1.0"}
echo "Building Environment Checker LSP v${VERSION}..."

mkdir -p release

# Linux x86_64
echo "Building for Linux x86_64..."
cd lsp
cargo build --release --target x86_64-unknown-linux-gnu
cd ..
mkdir -p release/env-checker-lsp-${VERSION}-x86_64-unknown-linux-gnu
cp lsp/target/x86_64-unknown-linux-gnu/release/env-checker-lsp release/env-checker-lsp-${VERSION}-x86_64-unknown-linux-gnu/
cd release/env-checker-lsp-${VERSION}-x86_64-unknown-linux-gnu
tar czf ../env-checker-lsp-${VERSION}-x86_64-unknown-linux-gnu.tar.gz env-checker-lsp
cd ../..

# macOS ARM64 (Apple Silicon)
echo "Building for macOS ARM64..."
cd lsp
cargo build --release --target aarch64-apple-darwin
cd ..
mkdir -p release/env-checker-lsp-${VERSION}-aarch64-apple-darwin
cp lsp/target/aarch64-apple-darwin/release/env-checker-lsp release/env-checker-lsp-${VERSION}-aarch64-apple-darwin/
cd release/env-checker-lsp-${VERSION}-aarch64-apple-darwin
tar czf ../env-checker-lsp-${VERSION}-aarch64-apple-darwin.tar.gz env-checker-lsp
cd ../..

# macOS x86_64 (Intel)
echo "Building for macOS x86_64..."
cd lsp
cargo build --release --target x86_64-apple-darwin
cd ..
mkdir -p release/env-checker-lsp-${VERSION}-x86_64-apple-darwin
cp lsp/target/x86_64-apple-darwin/release/env-checker-lsp release/env-checker-lsp-${VERSION}-x86_64-apple-darwin/
cd release/env-checker-lsp-${VERSION}-x86_64-apple-darwin
tar czf ../env-checker-lsp-${VERSION}-x86_64-apple-darwin.tar.gz env-checker-lsp
cd ../..

# Windows x86_64
echo "Building for Windows x86_64..."
cd lsp
cargo build --release --target x86_64-pc-windows-msvc
cd ..
mkdir -p release/env-checker-lsp-${VERSION}-x86_64-pc-windows-msvc/target/release
cp lsp/target/x86_64-pc-windows-msvc/release/env-checker-lsp.exe release/env-checker-lsp-${VERSION}-x86_64-pc-windows-msvc/target/release/
cd release/env-checker-lsp-${VERSION}-x86_64-pc-windows-msvc
zip -r ../env-checker-lsp-${VERSION}-x86_64-pc-windows-msvc.zip target/
cd ../..

echo "LSP binaries built successfully in release/ directory"
ls -lh release/*.tar.gz release/*.zip 2>/dev/null || true