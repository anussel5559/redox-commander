use std::{io::Read, ops::Deref};

use serde::de::DeserializeOwned;
use tracing::error;

pub mod paths;

/// Parse bytes from a reader into YAML. This will merge any anchors/aliases.
pub fn parse_yaml<T: DeserializeOwned>(reader: impl Read) -> serde_yaml::Result<T> {
    // Two-step parsing is required for anchor/alias merging
    let mut yaml_value: serde_yaml::Value = serde_yaml::from_reader(reader)?;
    yaml_value.apply_merge()?;
    serde_yaml::from_value(yaml_value)
}

/// Extension trait for [Result]
pub trait ResultTraced<T, E>: Sized {
    /// If this is an error, trace it. Return the same result.
    fn traced(self) -> Self;
}

// This is deliberately *not* implemented for non-anyhow errors, because we only
// want to trace errors that have full context attached
impl<T> ResultTraced<T, anyhow::Error> for anyhow::Result<T> {
    fn traced(self) -> Self {
        if let Err(err) = &self {
            error!(error = err.deref());
        }
        self
    }
}
