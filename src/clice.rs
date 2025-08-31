use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct CliceExtension;

struct CliceBinary {
    path: String,
    resource_dir: String,
}

impl CliceExtension {
    fn find_clice_binary(&self, worktree: &Worktree) -> Result<CliceBinary> {
        if let Some(path_str) = worktree.which("clice") {
            let separator_pos = path_str.rfind(|c| c == '\\' || c == '/');

            if let Some(pos) = separator_pos {
                let parent_dir = &path_str[..pos];
                let resource_dir = format!("{}\\{}", parent_dir, "lib");

                Ok(CliceBinary {
                    path: path_str,
                    resource_dir,
                })
            } else {
                return Err(format!(
                    "clice found as `{}`,
                    but a full path is required to find the `lib` directory.",
                    path_str
                ));
            }
        } else {
            Err(
                "`clice` not found in your PATH. Please install it and add it to your system's PATH environment variable.".to_string()
            )
        }
    }
}

impl zed::Extension for CliceExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.find_clice_binary(worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: vec![
                "--resource-dir".to_string(),
                binary.resource_dir,
                "--mode".to_string(),
                "pipe".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CliceExtension);
