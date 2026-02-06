# Zed Extension Structure

## Directory Layout

```
env-checker-zed/
├── extension.toml                    # Extension metadata
├── Cargo.toml                        # Rust extension dependencies
├── build.sh                          # Build and install script
├── LICENSE                            # MIT License
├── README.md                          # Extension documentation
├── QUICKSTART.md                      # Quick start guide
├── .gitignore                        # Git ignore patterns
├── .zed/
│   └── settings.json               # Development settings
├── languages/
│   └── env/                       # Environment language support
│       ├── config.toml               # Language configuration
│       ├── highlights.scm             # Syntax highlighting
│       ├── brackets.scm              # Bracket matching
│       ├── indents.scm               # Indentation rules
│       └── outline.scm               # Code outline
├── src/
│   └── lib.rs                      # Extension implementation
└── example-project/                   # Example for testing
    ├── .env                       # Example .env file
    ├── .envchecker.json           # Example config
    ├── env.schema.yml             # YAML schema
    ├── README.md                   # Example docs
    └── src/
        └── schema.ts              # Zod schema
```

## Key Files Explained

### `extension.toml`

Defines the extension:
- `id`: Unique identifier (env-checker)
- `name`: Display name (Environment Checker)
- `version`: Version (0.1.0)
- Language server registration

### `languages/env/config.toml`

Defines the "Env" language for `.env` files:
- Associates `.env` extension with Env language
- Uses `bash` grammar (since .env is simple)
- Defines comment syntax (`#`)

### Tree-sitter Query Files

- `highlights.scm`: Syntax highlighting rules
- `brackets.scm`: Bracket matching (minimal for .env)
- `indents.scm`: Indentation rules
- `outline.scm`: Code outline structure

### `src/lib.rs`

Rust extension code:
- Implements `Extension` trait
- Provides `language_server_command` method
- Finds and launches `env-checker-lsp` binary

### `example-project/`

Demonstrates extension usage:
- Complete example with schema and .env file
- Shows missing variables detection
- Documents expected behavior

## Installation

### 1. Install LSP Binary

```bash
cd ../env-checker-lsp
cargo install --path .
```

### 2. Install Extension

```bash
cd env-checker-zed
./build.sh
```

Or manually:
```bash
zed install
```

## Development

### Local Development

1. Make changes to extension code
2. Run `zed install` to rebuild
3. Restart Zed to reload extension

### Testing with Example

1. Install extension
2. Open `example-project` in Zed
3. Open `.env` file
4. Should see diagnostics for missing variables

## Publishing

When ready to publish:

1. Push to GitHub repository
2. Update `repository` in `extension.toml`
3. Submit to Zed Extensions marketplace
4. Users can install via Extensions panel

## Dependencies

- `zed`: Zed extension API crate
- `which`: Finding binaries in PATH
- `env-checker-lsp`: The actual LSP binary (separate project)

## How It Works

1. **Language Registration**: Extension registers "Env" language for `.env` files
2. **LSP Launch**: When `.env` is opened, extension finds and launches `env-checker-lsp`
3. **Communication**: Zed communicates with LSP via stdin/stdout
4. **Features**: Diagnostics, hover, and code actions flow through LSP protocol

## Notes

- LSP binary must be in PATH for extension to work
- Extension uses Bash grammar for syntax (simpler than custom grammar)
- Schema discovery happens within LSP, not extension
- Extension focuses on providing LSP integration, not validation logic
