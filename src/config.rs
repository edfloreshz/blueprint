// SPDX-License-Identifier: GPL-3.0

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};

use crate::app::models::package::Package;

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    shells: Vec<Package>,
    languages: Vec<Package>,
    editors: Vec<Package>,
    libraries: Vec<Package>,
}
