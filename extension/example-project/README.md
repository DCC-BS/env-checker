# Example Project for Environment Checker

This is a sample project demonstrating the Environment Checker LSP extension for Zed.

## Files

- `src/schema.ts` - Zod schema defining required environment variables
- `env.schema.yml` - YAML schema (alternative format)
- `.env` - Environment variables file (note: missing some required vars)
- `.envchecker.json` - Configuration for the LSP

## Expected Behavior

When you open `.env` in Zed with the extension installed:

1. **Missing Variables** will be highlighted as errors:
   - `API_URL` is missing (defined in schema but not in .env)

2. **Hover Information** - Hover over any variable to see:
   - Type (string/boolean)
   - Description
   - Default value
   - Required status
   - Group

3. **Code Actions** - Press `Cmd+.` / `Ctrl+.` to see:
   - "Append missing environment variable(s)" - Adds `API_URL` to .env
   - "Create .env.example file" - Generates example file

## Testing the Extension

1. Install `env-checker-lsp` binary:
   ```bash
   cd ../env-checker-lsp
   cargo install --path .
   ```

2. Install the Zed extension:
   ```bash
   cd ..
   zed install env-checker-zed
   ```

3. Open this project in Zed:
   ```bash
   cd env-checker-zed/example-project
   zed .
   ```

4. Open `.env` file - you should see diagnostic for `API_URL`

5. Try the code actions to fix it!
