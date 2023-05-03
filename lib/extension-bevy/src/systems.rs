use bevy_ecs::system::Res;
use etheryal_extension_common::message::{GuestMessage, GuestMessageEnum};
use tracing::{error, trace, warn};

use crate::ExtensionGuest;

pub fn send_guest_message_events(guest: Res<ExtensionGuest>) {
    while let Some(message) = read_message() {
        if let Some(packets) = guest.guest_messages.get(&GuestMessage::type_id(&message)) {
            packets.push(message)
        } else {
            warn!(
                "Received a guest message for an unregistered message type: {}",
                message.type_name()
            );
        }
    }
}

fn read_message() -> Option<GuestMessageEnum> {
    // SAFETY: This is safe because the extension host will only allow calling this
    // function after setting the extension info.
    let len = unsafe { etheryal_extension_sys::recv_message() };
    if len == 0 {
        trace!("No message to read");
        return None;
    }

    let mut out = Vec::with_capacity(len);
    let mut buffer = [0; 1024];

    trace!("Reading message of {len} bytes");
    while out.len() < len {
        // SAFETY: This is safe because we have enough space in the read
        // buffer.
        let read =
            unsafe { etheryal_extension_sys::read_message_buf(buffer.len(), buffer.as_mut_ptr()) };
        out.extend_from_slice(&buffer[..read]);
    }

    match rmp_serde::from_slice::<GuestMessageEnum>(&buffer[..]) {
        Ok(message) => Some(message),
        Err(err) => {
            error!("Failed to deserialize message: {err}");
            None
        },
    }
}
