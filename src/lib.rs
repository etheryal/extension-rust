#![doc = include_str!("../README.md")]
#![deny(missing_docs, clippy::missing_safety_doc)]
pub use {
    etheryal_extension_bevy as plugin, etheryal_extension_common as common,
    etheryal_identifier as identifier, semver,
};
pub mod prelude;
