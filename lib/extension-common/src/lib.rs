//! Common types and traits for etheryal extension modules and the etheryal
//! extension host
#![deny(missing_docs, clippy::missing_safety_doc)]
use etheryal_identifier::NamespacedIdentifier;
use getset::Getters;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod message;

/// Information about an extension WebAssembly module. This must be sent from
/// the extension to the host when the extension is loaded and before sending
/// any other message.
#[derive(Clone, Debug, Deserialize, Serialize, TypedBuilder, Getters)]
#[getset(get = "pub")]
pub struct ExtensionModuleInfo {
    /// The human readable name of the extension
    name: String,
    /// The extension's module unique identifier
    identifier: NamespacedIdentifier,
    /// The extension's module version
    version: Version,
    /// The extension's module dependencies
    dependencies: Vec<ExtensionModuleDependency>,
    /// The extension's module description
    #[builder(default)]
    description: Option<String>,
}

/// Information about an extension WebAssembly module dependency
#[derive(Clone, Debug, Deserialize, Serialize, TypedBuilder, Getters)]
#[getset(get = "pub")]
pub struct ExtensionModuleDependency {
    /// The dependency's unique identifier
    identifier: NamespacedIdentifier,
    /// The dependency's required version
    version: VersionReq,
    /// Whether the dependency is optional
    #[builder(default)]
    optional: bool,
}
