// SPDX-License-Identifier: GPL-3.0

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::Page;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub source: Source,
    pub config: Vec<ConfigFile>,
    pub page: Page,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigFile {
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
