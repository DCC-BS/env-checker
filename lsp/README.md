# Environment Checker LSP

A Language Server Protocol (LSP) implementation that validates environment variables against schema definitions.

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

## Installation

```bash
cargo install --path .
```

## Configuration

Create a `.envchecker.json` file in your project root (optional):

```json
{
  "schemaFiles": [
    "src/config/schema.ts",
    "backend/config.py"
  ],
  "envFiles": [
    ".env",
    ".env.local"
  ],
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

## Schema Formats

### Zod (TypeScript)

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
});
```

### Pydantic (Python)

```python
from pydantic import Field

class Configuration:
    api_url: str = Field(description="The URL for API service")
    debug: bool = Field(description="Enable debug mode", default=False)
    optional_var: str = Field(description="Optional variable", default="default_value")
```

### YAML

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

## Usage

### Editor Setup

#### VS Code

Add to your `.vscode/settings.json`:

```json
{
  "languageserver": {
    "env": {
      "command": "env-checker-lsp",
      "filetypes": ["sh"],
      "roots": [".git", ".envchecker.json"]
    }
  }
}
```

#### Vim/Neovim

Using `nvim-lspconfig`:

```lua
require('lspconfig').env_checker.setup({
  cmd = { 'env-checker-lsp' },
  filetypes = { 'sh' },
  root_dir = require('lspconfig.util').root_pattern('.git', '.envchecker.json'),
})
```

### Validation Behavior

1. The LSP automatically discovers schemas in your workspace
2. Opens `.env` files are validated against all discovered schemas
3. Missing required variables are reported as errors
4. Variables not defined in any schema are reported as informational

### Code Actions

When diagnostics are available:

- **"Append X missing environment variable(s)"**: Adds missing variables to the end of the `.env` file
  - Variables are grouped by schema metadata (`envType`/`group`)
  - Includes descriptions and default values as comments

- **"Create .env.example file"**: Creates or updates `.env.example` with all schema variables

### Example `.env.example` Output

```env
# API Configuration
API_URL=  # The URL for API service

# Secrets
API_KEY=  # API key for authentication (required)

# Settings
DEBUG=false  # Enable debug mode
```

## License

MIT
