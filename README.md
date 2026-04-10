# Dexter for Zed

[Dexter](https://github.com/remoteoss/dexter) is a lightning fast, full-featured Elixir language server optimized for speed on large codebases. This extension integrates Dexter with [Zed](https://zed.dev), running **alongside** your existing Elixir LSP (ElixirLS, Lexical, Next LS) — use Dexter for fast navigation and your full LSP for diagnostics, completions, and refactoring.

## Features

- Go-to-definition for modules, functions, aliases, imports, and delegates
- All def forms supported — `def`, `defp`, `defmacro`, `defmacrop`, `defguard`, `defguardp`, `defdelegate`, `defprotocol`, `defimpl`, `defstruct`, `defexception`
- Monorepo-aware with automatic reindexing on git branch switches

See the [Dexter repo](https://github.com/remoteoss/dexter) for the full feature list.

## Installation

1. [Install the Dexter binary](https://github.com/remoteoss/dexter#installation)
2. In Zed, open **Extensions** (`Cmd+Shift+X`) and search for **"Dexter"**
3. Click **Install**
4. Open any Elixir file — the index builds automatically on first startup

Add `.dexter.db` to your project's `.gitignore`:

```sh
echo ".dexter.db*" >> .gitignore
```

## Configuration

Configure Dexter in your Zed `settings.json` under the `lsp` key:

```json
{
  "lsp": {
    "dexter": {
      "binary": {
        "path": "/path/to/dexter",
        "arguments": ["lsp"]
      },
      "initialization_options": {
        "followDelegates": true
      }
    }
  }
}
```

| Setting | Default | Description |
|---------|---------|-------------|
| `binary.path` | `"dexter"` | Path to the Dexter binary. Defaults to finding `dexter` on your PATH. If you manage dexter with mise, prefer the shim path (`~/.local/share/mise/shims/dexter`) over a version-specific path. |
| `binary.arguments` | `["lsp"]` | Arguments passed to the binary. |
| `initialization_options.followDelegates` | `true` | Follow `defdelegate` to the target function definition. |

## Development

For contributing to this extension. Requires Rust installed via [rustup](https://rustup.rs/).

```sh
git clone https://github.com/remoteoss/dexter-zed.git
cd dexter-zed
cargo build --target wasm32-wasip1 --release
```

Then in Zed: `Cmd+Shift+P` → **"zed: install dev extension"** → select the `dexter-zed/` directory.

To view extension logs, run Zed from the terminal:

```sh
zed --foreground
```

Or open the log file via `Cmd+Shift+P` → **"zed: open log"**.

## License

MIT
