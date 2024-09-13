// SPDX-License-Identifier: GPL-3.0

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    name: String,
    source: Source,
    config: PackageConfig,
    enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageConfig {
    content: String,
    extension: String,
    target: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Source {
    Apt(String),
    Dnf(String),
    Pacman(String),
    Flatpak { id: String, version: String },
}
