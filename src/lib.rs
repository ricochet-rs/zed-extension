//! Launches `ricochet lsp` and `ricochet mcp` for Zed, leaving every rule to the CLI.

use serde::Deserialize;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, LanguageServerId, Project,
    Result, Worktree, serde_json, settings::ContextServerSettings,
};

const CLI: &str = "ricochet";
const CLI_INSTALL_URL: &str = "https://docs.ricochet.rs/latest/user/deployment/1-ricochet-cli/";
const DEFAULT_MCP_SETTINGS: &str = "{\n  \"server\": \"\"\n}";

/// The `settings` object of the ricochet context server in Zed's `settings.json`.
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct McpSettings {
    /// A server profile name or URL, passed to the CLI as `--server`.
    server: Option<String>,
}

impl McpSettings {
    /// Arguments for `ricochet`, selecting the configured server when one is set.
    fn args(&self) -> Vec<String> {
        let mut args = vec!["mcp".to_string()];
        if let Some(server) = self.server.as_deref().filter(|server| !server.is_empty()) {
            args.extend(["--server".to_string(), server.to_string()]);
        }
        args
    }
}

struct RicochetExtension;

impl zed::Extension for RicochetExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let command = worktree.which(CLI).ok_or_else(|| {
            format!("Install the ricochet CLI and put it on PATH: {CLI_INSTALL_URL}")
        })?;
        Ok(Command {
            command,
            args: vec!["lsp".to_string()],
            env: Vec::new(),
        })
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        let mcp_settings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|error| {
                format!("Invalid settings for the ricochet context server: {error}")
            })?,
            None => McpSettings::default(),
        };
        let command = settings
            .command
            .and_then(|command| command.path)
            .unwrap_or_else(|| CLI.to_string());
        Ok(Command {
            command,
            args: mcp_settings.args(),
            env: Vec::new(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = format!(
            "Install the ricochet CLI and put it on PATH: {CLI_INSTALL_URL}\n\n\
             Leave `server` empty to use your default server profile, \
             or set it to a profile name or a server URL."
        );
        let settings_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "server": {
                    "type": "string",
                    "description": "Server profile name or URL. Leave empty to use the default profile."
                }
            },
            "additionalProperties": false
        });
        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            settings_schema: settings_schema.to_string(),
            default_settings: DEFAULT_MCP_SETTINGS.to_string(),
        }))
    }
}

zed::register_extension!(RicochetExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(json: &str) -> McpSettings {
        serde_json::from_str(json).expect("valid settings JSON")
    }

    #[test]
    fn args_without_server_use_default_profile() {
        assert_eq!(parse("{}").args(), ["mcp"]);
    }

    #[test]
    fn args_with_empty_server_use_default_profile() {
        assert_eq!(parse(r#"{"server": ""}"#).args(), ["mcp"]);
    }

    #[test]
    fn args_with_server_pass_it_through() {
        assert_eq!(
            parse(r#"{"server": "prod"}"#).args(),
            ["mcp", "--server", "prod"]
        );
    }

    #[test]
    fn default_settings_use_default_profile() {
        assert_eq!(parse(DEFAULT_MCP_SETTINGS).args(), ["mcp"]);
    }
}
