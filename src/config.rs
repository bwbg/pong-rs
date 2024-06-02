//! This module is all about providing the application's configuration.
use crate::prelude::*;

/// Composition of the application's configuration directives.
#[derive(Debug)]
pub struct Configuration {
    /// display's width and height
    pub display_size: (u32, u32),
    /// diplay's title
    pub display_title: String,
    /// display's frames per second (refresh rate)
    pub display_fps: u32,
}

impl Default for Configuration {
    /// Create application's default configuration.
    fn default() -> Self {
        Configuration {
            display_size: (800, 600),
            display_title: "Pong".into(),
            display_fps: 60,
        }
    }
}

/// Provide the application's configuration.
/// TODO: Right now only the default configuration will be returned. In the
/// future this function should read a configuration file.
pub fn get() -> Result<Configuration> {
    Ok(Configuration::default())
}
