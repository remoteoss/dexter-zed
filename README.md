# Dexter for Zed

A [Zed](https://zed.dev) extension that provides fast Elixir go-to-definition via the [Dexter](https://gitlab.com/remote-com/employ-starbase/dexter) LSP server.

Dexter indexes every module and function definition in your Elixir project into a local SQLite database, then serves instant go-to-definition over LSP. It runs **alongside** your existing Elixir LSP (ElixirLS, Lexical, Next LS) — use Dexter for fast navigation and your full LSP for diagnostics, completions, and refactoring.

## Prerequisites

Install the Dexter CLI:

```sh
# Install dependencies
brew install sqlite
mise use -g go@1.26.1

# Install dexter
mise plugin add dexter git@gitlab.com:remote-com/employ-starbase/dexter.git
mise use -g dexter@latest
```

Verify it's on your PATH:

```sh
dexter --help
```

## Install the extension

### From source (dev extension)

1. Clone this repository:
   ```sh
   git clone git@gitlab.com:remote-com/employ-starbase/dexter-zed.git
   ```
2. In Zed, open the command palette (`Cmd+Shift+P`) and run **"zed: install dev extension"**
3. Select the `dexter-zed/` directory

### From the extension registry

*Not available (yet)*

## Configuration

Configure Dexter in your Zed `settings.json` under the `lsp` key:

```json
{
  "lsp": {
    "dexter": {
      "binary": {
        "path": "/path/to/dexter"
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
| `binary.path` | `"dexter"` | Path to the Dexter binary. Defaults to finding `dexter` on your PATH. If you manage dexter with mise, prefer the shim path (`/Users/yourname/.local/share/mise/shims/dexter`) over a version-specific path — this way the correct version is always resolved automatically when you update. |
| `binary.arguments` | `["lsp"]` | Arguments passed to the binary. |
| `initialization_options.followDelegates` | `true` | When jumping to a `defdelegate`, follow through to the target function. |

## Usage

1. Open an Elixir project in Zed. Dexter will automatically build the index on first startup (creates `.dexter.db` at the project root).
2. Use go-to-definition (`Cmd+Click` or `F12`) on any module or function — Dexter responds alongside your other LSP servers.

Add `.dexter.db*` to your `.gitignore`:

```sh
echo ".dexter.db*" >> .gitignore
```

## Features

- **Alias resolution** — `alias A.B.C`, `alias A.B.C, as: D`, `alias A.B.{Foo, Bar}`
- **Import resolution** — bare function calls resolved through `import` declarations
- **Delegate following** — `defdelegate fetch(id), to: Repo` jumps to `Repo.fetch`
- **Local buffer search** — private functions resolve without leaving the current file
- **All def forms** — `def`, `defp`, `defmacro`, `defmacrop`, `defguard`, `defguardp`, `defdelegate`, `defprotocol`, `defimpl`, `defstruct`, `defexception`
- **Git branch detection** — automatically reindexes when you switch branches
- **Parallel indexing** — uses all CPU cores for the initial index

## Development

Requires Rust installed via [rustup](https://rustup.rs/).

```sh
# Build the WASM extension
cargo build --target wasm32-wasip1 --release

# Install locally in Zed
# Cmd+Shift+P → "zed: install dev extension" → select this directory
```

To view extension logs, run Zed from the terminal:

```sh
zed --foreground
```

Or open the log file via `Cmd+Shift+P` → **"zed: open log"**.

## License

MIT
