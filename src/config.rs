// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 3]
pub struct Config {
    pub feeds: Vec<String>,
    pub refresh_interval_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            feeds: vec![
                "https://news.google.com/rss?hl=en-US&gl=US&ceid=US:en".to_string(),
            ],
            refresh_interval_seconds: 300,
        }
    }
}
