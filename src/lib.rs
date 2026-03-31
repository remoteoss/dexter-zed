use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result, Worktree};

struct DexterExtension;

impl zed::Extension for DexterExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let command = settings
            .binary
            .as_ref()
            .and_then(|b| b.path.clone())
            .unwrap_or_else(|| "dexter".to_string());

        let args = settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .filter(|a| !a.is_empty())
            .unwrap_or_else(|| vec!["lsp".to_string()]);

        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(options) = settings.initialization_options {
            return Ok(Some(options));
        }

        Ok(Some(zed::serde_json::json!({
            "followDelegates": true
        })))
    }
}

zed::register_extension!(DexterExtension);
