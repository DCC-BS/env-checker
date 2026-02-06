# Environment Checker - Zed Extension

A Zed extension that validates `.env` files against schema definitions.

## Features

- **Schema Support**:
  - **Zod (TypeScript)**: Parses Zod schema definitions
  - **Pydantic (Python)**: Parses Pydantic class definitions
  - **YAML**: Simple custom YAML format

- **Validation**:
  - Checks for missing required environment variables
  - Merges multiple `.env` files and validates against all schemas
  - Reports errors with detailed diagnostics

- **Code Actions**:
  - Append Missing Variables: Adds all missing required environment variables
  - Generate `.env.example`: Creates an example file with all schema variables

- **Hover Support**: Shows type, description, default value when hovering over env vars

## Installation

### Prerequisites

1. Install the Environment Checker LSP binary:
   ```bash
   cargo install env-checker-lsp
   ```
   Or build from source:
   ```bash
   cd /path/to/env-checker-lsp
   cargo build --release
   ```
   Then add the binary to your PATH:
   ```bash
   export PATH="$PATH:/path/to/env-checker-lsp/target/release"
   ```

2. Install this Zed extension:
   - Open Zed
   - Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
   - Search for "Environment Checker"
   - Click "Install"

Or manually install:
   ```bash
   cd env-checker-zed
   zed install
   ```

## Usage

### Creating a Schema

Create a schema file in your project. The LSP automatically discovers:

#### Zod (TypeScript)

```typescript
import { z } from "zod";

export const schema = z.object({
    API_KEY: z.string().describe("API key for authentication"),
    DEBUG: z.boolean().default(false).describe("Enable debug mode"),
});
```

#### Pydantic (Python)

```python
from pydantic import Field

class Configuration:
    api_url: str = Field(description="The URL for API service")
    debug: bool = Field(description="Enable debug mode", default=False)
```

#### YAML

Create `env.schema.yml` in your project:

```yaml
variables:
  API_KEY:
    type: string
    description: "API key for authentication"
    required: true
  DEBUG:
    type: boolean
    description: "Enable debug mode"
    default: false
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
- `envFiles`: Glob patterns for `.env` files to monitor
- `autoDiscover`: Automatically discover schema files (default: `true`)
- `groups`: Custom group name mappings

## Troubleshooting

### LSP not starting

1. Verify `env-checker-lsp` is in your PATH:
   ```bash
   which env-checker-lsp
   ```
2. Check Zed's log file for errors:
   - Open Zed
   - Press `Cmd+Shift+P` / `Ctrl+Shift+P`
   - Run "zed: Open Log"

### No diagnostics appearing

1. Ensure you have a schema file in your project
2. Check that `.env` file has `.env` extension
3. Reload the window: `Cmd+R` / `Ctrl+R`

## Development

### Building the extension

```bash
cd env-checker-zed
zed install
```

This will build and install the extension locally.

## License

MIT
