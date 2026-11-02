# CLAUDE.md

## What this repository is

A Zed extension that launches two subcommands of the `ricochet` CLI: `ricochet lsp` for `_ricochet.toml` diagnostics and `ricochet mcp` for the Agent Panel tools.
It only builds the command line, so put any validation, schema, server or tool logic in `ricochet-rs/cli`, where VS Code and other editors share it.
The language server is registered for all of `TOML`, which is correct only because `ricochet lsp` ignores every file not named `_ricochet.toml`.

## Zed API

`zed_extension_api` must stay within the range the oldest supported Zed accepts, so check the compatibility table in its README before raising it.
Zed builds extensions for `wasm32-wasip2`, so lint and build for that target, not the host.
The `mcp` arguments live in `McpSettings`, which is plain Rust, so test them with `cargo test` on the host.

## Validation

Run what `.crow/lint.yaml` runs before opening a pull request:

```sh
cargo fmt --check
cargo clippy --target wasm32-wasip2 -- -D warnings
cargo test
cargo build --target wasm32-wasip2 --release
```

Installing the extension needs Zed with a display: run **zed: install dev extension** and pick this directory.
