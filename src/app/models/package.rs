// SPDX-License-Identifier: GPL-3.0

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::Page;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub source: Source,
    pub config: Vec<ConfigFile>,
    pub page: Page,
    pub enabled: bool,
}

impl Package {
    pub fn new(name: &str, source: Source, page: Page) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            source,
            config: vec![],
            page,
            enabled: true,
        }
    }
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
