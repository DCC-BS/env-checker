# Quick Start Guide

## Installation

### 1. Build and Install LSP Binary

```bash
cd env-checker-lsp
cargo install --path .
```

This installs `env-checker-lsp` binary to `~/.cargo/bin/`. Make sure it's in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this line to your `~/.zshrc` or `~/.bashrc` to make it permanent.

### 2. Install Zed Extension

```bash
cd env-checker-zed
zed install
```

This builds and installs the extension into Zed.

## Verification

1. Open Zed
2. Open the `example-project/.env` file
3. You should see red squiggly underline indicating `API_URL` is missing
4. Hover over `DEBUG` to see its schema information
5. Press `Cmd+.` (macOS) or `Ctrl+.` (Linux/Windows) to see code actions
6. Select "Append missing environment variable(s)" to fix the error

## What to Expect

### Syntax Highlighting

The extension provides syntax highlighting for `.env` files:
- Comments (`#`) are highlighted
- Variable names are highlighted
- Values are highlighted

### Diagnostics

Missing required variables appear as errors:
- Red underline
- Error message with description
- Default value if available

### Hover

Hover over any variable to see:
- **Type**: string/boolean/number
- **Description**: Schema description
- **Default**: Default value
- **Required**: true/false
- **Group**: Which group it belongs to

### Code Actions

Two actions available:
1. **Append missing environment variable(s)**
   - Adds all missing required vars
   - Groups by schema metadata
   - Includes descriptions as comments

2. **Create .env.example file**
   - Creates `.env.example` with all schema variables
   - Useful for documentation

## Troubleshooting

### Extension not loading

Check Zed's log:
1. Press `Cmd+Shift+P` / `Ctrl+Shift+P`
2. Type "open log" and select "zed: Open Log"
3. Look for errors related to "env-checker"

### LSP not starting

1. Verify binary is installed:
   ```bash
   which env-checker-lsp
   ```
   Should show path to binary

2. Test the LSP directly:
   ```bash
   env-checker-lsp --help
   ```

3. Check for PATH issues:
   ```bash
   echo $PATH | grep cargo
   ```

### No diagnostics appearing

1. Ensure you have a schema file:
   - Zod: `**/schema.ts`, `**/config.ts`
   - Pydantic: `**/config.py`, `**/settings.py`
   - YAML: `env.schema.yml`

2. Check file extension is `.env` (not `.env.local`, etc.)

3. Reload the window: `Cmd+R` / `Ctrl+R`

## Next Steps

1. Create a schema in your project
2. Add a `.env` file
3. Open in Zed to see validation
4. Use code actions to fix missing variables

For more details, see:
- [Extension README](./README.md)
- [LSP README](../env-checker-lsp/README.md)
