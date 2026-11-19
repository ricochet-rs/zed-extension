//! Launches `ricochet lsp` for Zed, leaving every rule to the CLI.

use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

const CLI: &str = "ricochet";
const CLI_INSTALL_URL: &str = "https://docs.ricochet.rs/latest/user/deployment/1-ricochet-cli/";

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
            format!("Install the ricochet CLI 1.5.0 or newer and put it on PATH: {CLI_INSTALL_URL}")
        })?;
        Ok(Command {
            command,
            args: vec!["lsp".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(RicochetExtension);
