# GitHub Workflows

This repository includes GitHub Actions workflows for automated CI/CD.

## Workflows

### CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch

**Jobs:**
1. **Test**: Runs LSP unit tests
2. **Build Extension**: Builds the Zed extension WASM module
3. **Build LSP**: Builds LSP binary for Linux and checks binary size

**Purpose:** Catch issues early before they reach release.

### Release Workflow (`.github/workflows/release.yml`)

**Triggers:**
- Push of a tag matching `v*.*.*` (e.g., `v0.1.0`)

**Jobs:**
1. **Build Extension**: Builds `extension.wasm` on Ubuntu
2. **Build LSP**: Builds LSP binaries for multiple platforms:
   - Linux x86_64 (Ubuntu latest)
   - macOS ARM64 (Apple Silicon) (macOS latest)
   - macOS x86_64 (Intel) (macOS 13)
   - Windows x86_64 (Windows latest)
3. **Release**: Creates GitHub release with all artifacts

**Artifacts:**
- `extension.wasm`: Zed extension module
- `env-checker-lsp-{version}-{arch}-{platform}.tar.gz` (Linux/macOS)
- `env-checker-lsp-{version}-{arch}-{platform}.zip` (Windows)

## Creating a Release

### 1. Update Version

Update the version in `Cargo.toml` workspace:

```toml
[workspace.package]
version = "0.2.0"  # Update to new version
```

### 2. Commit and Tag

```bash
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags
```

### 3. Monitor Workflow

GitHub Actions will automatically:
1. Build extension.wasm
2. Build LSP binaries for all platforms
3. Create a GitHub release with all artifacts
4. Generate release notes from commits

The release will be available at: `https://github.com/yourusername/env-checker/releases/tag/v0.2.0`

## Platform Matrix

| Platform | Architecture | OS in Workflow | Target |
|----------|--------------|-----------------|---------|
| Linux | x86_64 | ubuntu-latest | x86_64-unknown-linux-gnu |
| macOS | ARM64 | macos-latest | aarch64-apple-darwin |
| macOS | x86_64 | macos-13 | x86_64-apple-darwin |
| Windows | x86_64 | windows-latest | x86_64-pc-windows-msvc |

## Workflow Secrets

The workflows use the built-in `GITHUB_TOKEN` secret, which is automatically available. No additional secrets are required.

## Local Testing

Before pushing a tag, you can test the build locally:

```bash
# Test release build
./scripts/build-all.sh 0.2.0

# Verify artifacts are created
ls -lh release/*.tar.gz release/*.zip
ls -lh extension/extension.wasm
```

## Troubleshooting

### Workflow Fails on macOS Cross-Compilation

If macOS builds fail, ensure you're using:
- `macos-latest` for ARM64 (Apple Silicon)
- `macos-13` for x86_64 (Intel)

### Workflow Fails on Windows

Windows builds may fail if path separators are incorrect. The workflow uses PowerShell for Windows-specific commands.

### Binary Size Too Large

The CI workflow checks if the LSP binary exceeds 10MB. To reduce size:
- Remove unused dependencies
- Use `strip` on the binary
- Consider optimizing dependencies

### Extension Build Fails

If Zed extension build fails:
- Check that `zed-industries/setup-zed@v1` action is up to date
- Verify `Cargo.toml` has correct workspace configuration
- Check for compilation errors in `extension/src/lib.rs`