//! Global events that can be sent between the extension host and the extension
//! guest
use etheryal_extension_derive::ExtensionMessage;
use serde::{Deserialize, Serialize};

/// A message sent from the extension host to the extension guest
/// when the extension host is shutting down
#[derive(Serialize, Deserialize, Debug, Clone, ExtensionMessage)]
#[extension_message(guest)]
pub struct ShutdownGuest;

/// A message sent from the extension guest to the extension host
/// when the extension wants to shut down the extension host
/// (e.g. when the extension wants to close the game server for any reason)
#[derive(Serialize, Deserialize, Debug, Clone, ExtensionMessage)]
#[extension_message(host)]
pub struct ShutdownHost;
