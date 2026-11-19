# Ricochet for Zed

Check your `_ricochet.toml` as you type.
The diagnostics come from `ricochet lsp`, so Zed applies the same rules as `ricochet deploy`.

## Install

1. Install the [ricochet CLI](https://docs.ricochet.rs/latest/user/deployment/1-ricochet-cli/) 1.5.0 or newer and make sure `ricochet` is on your `PATH`.
2. In Zed, open **zed: extensions** from the command palette and install **TOML** and **Ricochet**.

Zed now shows problems in any `_ricochet.toml` you open.
Other TOML files are left alone.

## Deploy from the Agent Panel

The CLI also serves the ricochet tools to Zed's Agent Panel with `ricochet mcp`.
Zed adds MCP servers in its settings rather than through extensions, so add it once yourself.

1. Log in to your server with `ricochet login`.
2. Open **Settings > AI > MCP Servers**, choose **Add Server**, then **Add Local Server**, or add this to your `settings.json`:

```json
{
  "context_servers": {
    "ricochet": {
      "command": "ricochet",
      "args": ["mcp"]
    }
  }
}
```

Ask the agent to preview or deploy the project you have open.
The tools use your default server profile.
To pick another profile or a server URL, add `"--server", "production"` after `"mcp"` in `args`.

## Troubleshooting

If Zed reports that it cannot find the CLI, install it from the link above and restart Zed.
If the language server exits with "unrecognized subcommand 'lsp'", your CLI predates 1.5.0, so run `ricochet self update`.
Run **zed: open log** to see why the language server failed to start.

## License

[GPL-3.0](LICENSE)
