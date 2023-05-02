//! Debug messages used to test the extension host <-> extension guest
//! communication
use etheryal_extension_derive::ExtensionMessage;
use serde::{Deserialize, Serialize};

/// A message sent from the extension guest to the extension host
/// when the extension wants to send a test message and get a response back, in
/// this case, the extension will receive a `Pong` message (This is used to test
/// the extension host <-> extension guest communication)
#[derive(Serialize, Deserialize, Debug, Clone, ExtensionMessage)]
#[extension_message(host)]
pub struct Ping;

/// A message sent from the extension host to the extension guest
/// when the extension guest sends a `Ping` message, the host will respond with
/// a `Pong` message
#[derive(Serialize, Deserialize, Debug, Clone, ExtensionMessage)]
#[extension_message(guest)]
pub struct Pong;
