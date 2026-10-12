//! Embedded global styles for WoW Buddy.
//!
//! The CSS is precompiled from Tailwind (`input.css`) into `assets/app.css`
//! and compiled into the binary so the application looks styled 100% offline
//! across Desktop WebView, Web, iOS and Android.

/// Compiled Tailwind CSS bundle (precompiled via Tailwind CLI).
pub const APP_CSS: &str = include_str!("../assets/app.css");
