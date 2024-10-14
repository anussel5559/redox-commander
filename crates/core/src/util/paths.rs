use anyhow::{anyhow, Context};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Get the path of the directory to contain log files. **Directory
/// may not exist yet**, caller must create it.
pub fn log_directory() -> PathBuf {
    // State dir is only present on windows, but cache dir will be present on
    // all platforms
    // https://docs.rs/dirs/latest/dirs/fn.state_dir.html
    // https://docs.rs/dirs/latest/dirs/fn.cache_dir.html
    debug_or(
        dirs::state_dir()
            .unwrap_or_else(|| dirs::cache_dir().unwrap())
            .join("redox_commander"),
    )
}

/// Get the path to the primary log file. **Parent direct may not exist yet,**
/// caller must create it.
pub fn log_file() -> PathBuf {
    log_directory().join("redox_commander.log")
}

/// Get the path to the backup log file **Parent direct may not exist yet,**
/// caller must create it.
pub fn log_file_old() -> PathBuf {
    log_directory().join("redox_commander.log.old")
}

/// In debug mode, use a local directory for all files. In release, use the
/// given path.
fn debug_or(path: PathBuf) -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let _ = path; // Remove unused warning
        get_repo_root().join("data/")
    }
    #[cfg(not(debug_assertions))]
    {
        path
    }
}

/// Get path to the root of the git repo. This is needed because this crate
/// doesn't live at the repo root, so we can't use `CARGO_MANIFEST_DIR`. Path
/// will be cached so subsequent calls are fast. If the path can't be found,
/// fall back to the current working directory instead. Always returns an
/// absolute path.
#[cfg(any(debug_assertions, test))]
pub(crate) fn get_repo_root() -> &'static Path {
    use anyhow::Context;

    use crate::util::ResultTraced;
    use std::{process::Command, sync::OnceLock};

    static CACHE: OnceLock<PathBuf> = OnceLock::new();

    CACHE.get_or_init(|| {
        let try_get = || -> anyhow::Result<PathBuf> {
            let output = Command::new("git")
                .args(["rev-parse", "--show-toplevel"])
                .output()?;
            let path = String::from_utf8(output.stdout)?;
            Ok(path.trim().into())
        };
        try_get()
            .context("Error getting repo root path")
            .traced()
            .unwrap_or_default()
    })
}

/// Ensure the parent directory of a file path exists
pub fn create_parent(path: &Path) -> anyhow::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Cannot create directory for path {path:?}; it has no parent"))?;
    fs::create_dir_all(parent).context("Error creating directory {parent:?}")?;
    Ok(())
}
