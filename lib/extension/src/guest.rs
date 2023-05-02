use std::any::TypeId;

use bevy_ecs::prelude::*;
use crossbeam_queue::SegQueue;
use dashmap::DashMap;
use etheryal_extension_common::message::{GuestMessageEnum, HostMessageEnum};
use tracing::trace;

use crate::error::ExtensionError;

/// A Bevy resource that allows the extension guest to interact with the
/// extension host.
#[derive(Resource)]
pub struct ExtensionGuest {
    pub(crate) guest_messages: DashMap<TypeId, SegQueue<GuestMessageEnum>>,
}

impl ExtensionGuest {
    pub(crate) fn new() -> Self {
        Self {
            guest_messages: DashMap::new(),
        }
    }
}

impl ExtensionGuest {
    /// Send a message to the extension host
    pub fn send_message<H: Into<HostMessageEnum>>(&self, message: H) -> Result<(), ExtensionError> {
        send_message(message.into())
    }
}

fn send_message(message: HostMessageEnum) -> Result<(), ExtensionError> {
    let encoded = rmp_serde::to_vec_named(&message)?;

    let len = encoded.len();
    trace!("Sending message of {len} bytes");

    // SAFETY: This is safe because the extension host will only call this function
    // after sending extension info.
    unsafe { etheryal_extension_sys::send_message(len, encoded.as_ptr()) };
    Ok(())
}
