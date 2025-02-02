//! # The `nightly` Module
//!
//! This module contains configuration for opt-in nightly unstable features that are in the testing
//! phase.
//!
//! This API is very unstable and may change rapidly as features are added into the bot.

use serde::Deserialize;

/// # Struct `NightlyFeatures`
///
/// The opt-in nightly features that the bot provides.
#[derive(Default, Deserialize)]
pub struct NightlyFeatures {
    // Experimental Support for the Discord Threads API
    #[serde(default = "default_feature_enabled")]
    pub threads: bool,
    // Experimental Support for localization Facilities, i.e. timezones, languages
    #[serde(default = "default_feature_enabled")]
    pub localization: bool
}

pub fn default_feature_enabled() -> bool {
    false
}
