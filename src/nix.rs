use zed_extension_api::{self as zed, Result,LanguageServerId};

struct NixExtension {
}

impl zed::Extension for NixExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("nil")
            .ok_or_else(|| "The Nix language server (nil) is not available in your environment (PATH).
                You can install it from https://github.com/oxalica/nil.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }

}

zed::register_extension!(NixExtension);
