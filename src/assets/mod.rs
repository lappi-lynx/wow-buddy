//! Offline asset registry.
//!
//! Asset metadata lives in `assets/manifest.json`; the actual bytes are
//! generated into `generated.rs` as `data:` URIs by `tools/assets`, so the
//! app ships 100% offline with no runtime file or network access.

pub mod generated;
pub mod registry;
pub mod zone;

pub use registry::{AssetEntry, AssetRegistry, ICON_FALLBACK};
