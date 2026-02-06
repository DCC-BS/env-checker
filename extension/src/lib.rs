use std::{fs, path::Path};
use zed_extension_api as zed;

struct EnvChecker {
    cached_binary_path: Option<String>,
}

impl EnvChecker {
    fn language_server_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        if let Some(path) = worktree.which("env-checker-lsp") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "DCC-BS/env-checker",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, architecture) = zed::current_platform();
        let version = release.version.clone();

        let asset_name = Self::binary_release_name(&version, &platform, &architecture);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("env-checker-lsp-{}", version);
        let binary_path = Path::new(&version_dir)
            .join(Self::binary_path_within_archive(&platform, &architecture))
            .to_str()
            .expect("Could not convert binary path to str")
            .to_string();

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let file_kind = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };
            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            Self::clean_other_installations(&version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn binary_release_name(
        version: &String,
        platform: &zed::Os,
        architecture: &zed::Architecture,
    ) -> String {
        format!(
            "env-checker-lsp-{version}-{arch}-{os}.{ext}",
            version = version,
            arch = match architecture {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 | zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            },
            ext = match platform {
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        )
    }

    fn binary_path_within_archive(platform: &zed::Os, architecture: &zed::Architecture) -> String {
        let path = match platform {
            zed::Os::Windows => Path::new("target")
                .join(format!(
                    "{arch}-pc-windows-msvc",
                    arch = match architecture {
                        zed::Architecture::Aarch64 => "aarch64",
                        zed::Architecture::X86 | zed::Architecture::X8664 => "x86_64",
                    },
                ))
                .join("release")
                .join("env-checker-lsp.exe")
                .as_path()
                .to_owned(),
            _ => Path::new("env-checker-lsp").to_owned(),
        };
        path.to_str()
            .expect("Could not convert binary path to str")
            .to_string()
    }

    fn clean_other_installations(version_to_keep: &String) -> Result<(), String> {
        let entries =
            fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
            if entry.file_name().to_str() != Some(version_to_keep) {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
        Ok(())
    }
}

impl zed::Extension for EnvChecker {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let binary_path = self
            .language_server_binary(language_server_id, worktree)
            .map_err(|e| format!("failed to get binary path: {e}"))?;

        Ok(zed::Command {
            command: binary_path,
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(EnvChecker);
