# Ricochet for Zed

Check your `_ricochet.toml` as you type, and deploy to ricochet from Zed's Agent Panel.

## Install

1. Install the [ricochet CLI](https://docs.ricochet.rs/latest/user/deployment/1-ricochet-cli/) and make sure `ricochet` is on your `PATH`.
2. Log in to your server with `ricochet login`.
3. In Zed, open **zed: extensions** from the command palette and install **TOML** and **Ricochet**.

Zed now shows problems in any `_ricochet.toml` you open.
Other TOML files are left alone.

## Deploy from the Agent Panel

The Agent Panel lists the ricochet tools once the extension is installed.
Ask the agent to preview or deploy the project you have open.

The tools use your default server profile.
To pick another profile or a server URL, set `server` in Zed's `settings.json`:

```json
{
  "context_servers": {
    "ricochet": {
      "settings": {
        "server": "production"
      }
    }
  }
}
```

## Troubleshooting

If Zed reports that it cannot find the CLI, install it from the link above and restart Zed.
If `ricochet` lives outside the `PATH` Zed sees, set its location for the Agent Panel tools:

```json
{
  "context_servers": {
    "ricochet": {
      "command": {
        "path": "/path/to/ricochet"
      }
    }
  }
}
```

## License

[GPL-3.0](LICENSE)
