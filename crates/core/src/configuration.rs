use anyhow::{anyhow, Context};
use itertools::Itertools;
use std::{
    env,
    future::Future,
    path::{Path, PathBuf},
};
use tokio::task;
use tracing::{trace, warn};

mod model;
pub use model::*;

/// The support file names to be automatically loaded as a config. We only
/// support loading from one file at a time, so if more than one of these is
/// defined, we'll take the earliest and print a warning.
const CONFIG_FILES: &[&str] = &[
    "rc.yml",
    "rc.yaml",
    ".rc.yml",
    ".rc.yaml",
    "redox_commander.yml",
    "redox_commander.yaml",
    ".redox_commander.yml",
    ".redox_commander.yaml",
];

/// A wrapper around a configuration to handle functionality around the
/// file system.
#[derive(Debug)]
pub struct ConfigurationFile {
    path: PathBuf,
    pub configuration: Configuration,
}

impl ConfigurationFile {
    /// Create a new configuration file with the given path and a default
    /// configuration. Useful when the configuraiton failed to load and you want a
    /// placeholder.
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            path,
            configuration: Default::default(),
        }
    }

    /// Load config from the given file. The caller is responsible for using
    /// [Self::try_path] to find the file themself. This pattern enables the
    /// TUI to start up and watch the collection file, even if it's invalid.
    pub async fn load(path: PathBuf) -> anyhow::Result<Self> {
        let configuration = load_configuration(path.clone()).await?.into();
        Ok(Self {
            path,
            configuration,
        })
    }

    /// Reload a new collection from the same file used for this one.
    ///
    /// Returns `impl Future` to unlink the future from `&self`'s lifetime.
    pub fn reload(&self) -> impl Future<Output = anyhow::Result<Configuration>> {
        load_configuration(self.path.clone())
    }

    /// Get the path of the file that this collection was loaded from
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the path to the configuration file, returning an error if none is
    /// available. This will use the override if given, otherwise it will fall
    /// back to searching the given directory for a configuration. If the directory
    /// to search is not given, default to the current directory.
    pub fn try_path(
        dir: Option<PathBuf>,
        override_path: Option<PathBuf>,
    ) -> anyhow::Result<PathBuf> {
        let dir = if let Some(dir) = dir {
            dir
        } else {
            env::current_dir()?
        };
        override_path
            .map(|override_path| dir.join(override_path))
            .or_else(|| detect_path(&dir))
            .ok_or_else(|| {
                anyhow!("No configuration file found in current or ancestor directories")
            })
    }
}

/// Search the current directory for a config file matching one of the known
/// file names, and return it if found
fn detect_path(dir: &Path) -> Option<PathBuf> {
    /// Search a directory and its parents for the configuration file. Return None
    /// only if we got through the whole tree and couldn't find it
    fn search_all(dir: &Path) -> Option<PathBuf> {
        search(dir).or_else(|| {
            let parent = dir.parent()?;
            search_all(parent)
        })
    }

    /// Search a single directory for a configuration file
    fn search(dir: &Path) -> Option<PathBuf> {
        trace!("Scanning for coinfiguration file in {dir:?}");
        let paths = CONFIG_FILES
            .iter()
            .map(|file| dir.join(file))
            .filter(|p| p.exists())
            .collect_vec();
        match paths.as_slice() {
            [] => None,
            [first, rest @ ..] => {
                if !rest.is_empty() {
                    warn!(
                        "Multiple configuration files detected. {first:?} will be \
                            used and the following will be ignored: {rest:?}"
                    );
                }

                trace!("Found configuration file at {first:?}");
                Some(first.to_path_buf())
            }
        }
    }

    // Walk *up* the tree until we've hit the root
    search_all(dir)
}

/// Load a configuration from the given file. Takes an owned path because it
/// needs to be passed to a future
async fn load_configuration(path: PathBuf) -> anyhow::Result<Configuration> {
    // YAML parsing is blocking so do it in a different thread. We could use
    // tokio::fs for this but that just uses std::fs underneath anyway.
    task::spawn_blocking(move || Configuration::load(&path))
        .await
        // This error only occurs if the task panics
        .context("Error parsing configuration")?
}
