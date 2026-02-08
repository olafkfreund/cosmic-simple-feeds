// SPDX-License-Identifier: MPL-2.0

mod app;
mod config;
mod error;
mod feed;
mod i18n;

use cosmic::Application;
use cosmic::cosmic_config::CosmicConfigEntry;

fn main() -> cosmic::iced::Result {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    let config = match cosmic::cosmic_config::Config::new(
        app::AppModel::APP_ID,
        crate::config::Config::VERSION,
    ) {
        Ok(cosmic_cfg) => match crate::config::Config::get_entry(&cosmic_cfg) {
            Ok(cfg) => cfg,
            Err((_errors, default)) => default,
        },
        Err(error) => {
            eprintln!("Failed to open cosmic config: {error}");
            Default::default()
        }
    };

    // Starts the applet's event loop with the loaded configuration as flags.
    cosmic::applet::run::<app::AppModel>(config)
}
