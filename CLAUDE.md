# CLAUDE.md

## What this repository is

A Zed extension that launches `ricochet lsp` for `_ricochet.toml` diagnostics.
It only builds the command line, so put any validation or schema logic in `ricochet-rs/cli`, where VS Code and other editors share it.
It has no context server, because Zed's registry no longer accepts MCP server extensions, so the README shows users how to add `ricochet mcp` in Zed's settings instead.
The registry asks for an `-lsp` id suffix on an extension that only provides a language server, so keep `ricochet-lsp` unless the extension gains another kind of feature.
The language server is registered for all of `TOML`, which is correct only because `ricochet lsp` ignores every file not named `_ricochet.toml`.

## Zed API

`zed_extension_api` must stay within the range the oldest supported Zed accepts, so check the compatibility table in its README before raising it.
Zed builds extensions for `wasm32-wasip2`, so lint and build for that target, not the host.

## Validation

Run what `.crow/lint.yaml` runs before opening a pull request:

```sh
cargo fmt --check
cargo clippy --target wasm32-wasip2 -- -D warnings
cargo test
cargo build --target wasm32-wasip2 --release
```

Installing the extension needs Zed with a display: run **zed: install dev extension** and pick this directory.

## Publishing

Zed's extension registry does not accept pull requests from autonomous agents, and it wants a human to write the pull request and its replies, so leave that submission to a maintainer.
The `version` in the registry's `extensions.toml` must match `extension.toml` at the submitted commit, so raise both together.
