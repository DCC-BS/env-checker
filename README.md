# Environment Checker

A comprehensive environment variable validation solution for Zed, combining a Language Server Protocol (LSP) implementation with a Zed editor extension.

## Features

- **Schema Support**:
  - **Zod (TypeScript)**: Parses Zod schema definitions with `.describe()`, `.default()`, and `.register()` metadata
  - **Pydantic (Python)**: Parses Pydantic class definitions with `Field()` descriptions and defaults
  - **YAML**: Simple custom YAML format for schema definitions

- **Validation**:
  - Checks for missing required environment variables
  - Merges multiple `.env` files and validates against all schemas
  - Reports errors with detailed diagnostics

- **Code Actions**:
  - **Append Missing Variables**: Adds all missing required environment variables to the current `.env` file
  - **Generate `.env.example`**: Creates an example file with all schema variables

- **Hover Support**: Shows type, description, default value, and group information when hovering over environment variables

## Repository Structure

This is a monorepo containing both the LSP implementation and the Zed extension:

```
env-checker/
├── extension/          # Zed extension (WASM)
│   ├── src/
│   ├── extension.toml
│   └── languages/
├── lsp/                # LSP binary (Rust)
│   └── src/
├── scripts/           # Build scripts
│   ├── build-extension.sh
│   ├── build-lsp-all.sh
│   └── build-all.sh
└── Cargo.toml        # Workspace configuration
```

## Installation

### Prerequisites

- [Zed editor](https://zed.dev/)
- [Rust toolchain](https://rustup.rs/) (for development)

### Installing the Extension

#### From Zed Extensions Marketplace

1. Open Zed
2. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
3. Search for "Environment Checker"
4. Click "Install"

#### From Source

```bash
git clone https://github.com/yourusername/env-checker.git
cd env-checker/extension
zed install
```

The extension will automatically download the appropriate LSP binary on first use.

## Usage

### Creating a Schema

Create a schema file in your project. The LSP automatically discovers:

#### Zod (TypeScript)

```typescript
import { z } from "zod";

type EnvType = "build-time" | "runtime";

export const envRegistry = z.registry<{ envType: EnvType; group: string }>();

export const schema = z.object({
    mySecret: z.string()
        .default("defaultValue")
        .describe("This is a secret value")
        .register(envRegistry, { envType: "runtime", group: "Secrets" }),
    debug: z.boolean()
        .default(false)
        .describe("Enable debug mode")
        .register(envRegistry, { envType: "build-time", group: "Settings" }),
    apiUrl: z.string()
        .describe("API endpoint URL")
        .register(envRegistry, { envType: "runtime", group: "API" }),
});
```

#### Pydantic (Python)

```python
from pydantic import Field

class Configuration:
    api_url: str = Field(description="The URL for API service")
    debug: bool = Field(description="Enable debug mode", default=False)
    optional_var: str = Field(description="Optional variable", default="default_value")
```

#### YAML

Create `env.schema.yml` in your project:

```yaml
variables:
  API_KEY:
    type: string
    description: "API key for authentication"
    required: true
    group: "API"
  DEBUG:
    type: boolean
    description: "Enable debug mode"
    default: false
    group: "Settings"
```

### Using in Zed

1. Open a `.env` file in Zed
2. Missing required variables are highlighted as errors
3. Hover over variables to see their schema information
4. Use `Cmd+.` (macOS) or `Ctrl+.` (Linux/Windows) to see code actions:
   - "Append missing environment variable(s)": Adds missing vars to the file
   - "Create .env.example file": Generates an example file

## Configuration

Create `.envchecker.json` in your project root:

```json
{
  "schemaFiles": [
    "src/config/schema.ts",
    "backend/config.py"
  ],
  "envFiles": [".env", ".env.local"],
  "autoDiscover": true,
  "groups": {
    "runtime": "Runtime Variables",
    "build-time": "Build-time Variables"
  }
}
```

### Configuration Options

- `schemaFiles`: Array of explicit schema file paths
- `envFiles`: Glob patterns for `.env` files to monitor (default: `.env` in root)
- `autoDiscover`: Automatically discover schema files (default: `true`)
- `groups`: Custom group name mappings

## Development

### Building the Extension

```bash
./scripts/build-extension.sh
```

This builds the Zed extension as a WASM module.

### Building the LSP

```bash
./scripts/build-lsp-all.sh 0.1.0
```

This builds the LSP binary for all supported platforms:
- Linux (x86_64)
- macOS (ARM64 and x86_64)
- Windows (x86_64)

### Building Everything

```bash
./scripts/build-all.sh 0.1.0
```

This builds both the extension and all LSP binaries.

### Local Development

1. Make changes to the extension or LSP code
2. Build the extension: `./scripts/build-extension.sh`
3. In Zed: `zed install` (from the extension directory)
4. Reload Zed: `Cmd+R` / `Ctrl+R`
5. For LSP changes, rebuild with: `cargo build --release --manifest-path=lsp/Cargo.toml`

### Testing with Example

1. Install extension
2. Open `extension/example-project` in Zed
3. Open `.env` file
4. Should see diagnostics for missing variables

## Release Process

1. Update version in `Cargo.toml` workspace
2. Build all artifacts: `./scripts/build-all.sh <version>`
3. Create a new GitHub release with tag `v<version>`
4. Upload:
   - `extension/extension.wasm`
   - All `.tar.gz` and `.zip` files from `release/`
5. Update extension in Zed Extensions marketplace

## Troubleshooting

### LSP not starting

1. Check Zed's log: Press `Cmd+Shift+P` / `Ctrl+Shift+P`, then "zed: Open Log"
2. Look for errors related to "env-checker"
3. The extension should auto-download the LSP binary on first use

### No diagnostics appearing

1. Ensure you have a schema file in your project
2. Check that `.env` file has `.env` extension
3. Reload the window: `Cmd+R` / `Ctrl+R`

## License

MIT