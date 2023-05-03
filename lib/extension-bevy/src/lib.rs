//! A Bevy plugin that provides utilities for creating etheryal WebAssembly
//! extensions.
#![deny(missing_docs, clippy::missing_safety_doc)]
use std::any::{type_name, TypeId};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::event::EventWriter;
use bevy_ecs::system::Res;
use crossbeam_queue::SegQueue;
use error::ExtensionError;
use etheryal_extension_common::message::debug::Pong;
use etheryal_extension_common::message::events::ShutdownGuest;
use etheryal_extension_common::message::{GuestMessage, GuestMessageEnum};
use etheryal_extension_common::ExtensionModuleInfo;
pub use event::ExtensionEvent;
pub use guest::ExtensionGuest;
use tracing::{debug, warn};

mod error;
mod event;
mod guest;
mod systems;

/// A Bevy plugin that provides utilities for creating etheryal extensions.
pub struct EtheryalExtensionPlugin {
    guest_info: ExtensionModuleInfo,
}

impl EtheryalExtensionPlugin {
    /// Create a new plugin with the given extension module information
    pub fn new(guest_info: ExtensionModuleInfo) -> Self {
        Self { guest_info }
    }
}

impl Plugin for EtheryalExtensionPlugin {
    fn build(&self, app: &mut App) {
        set_extension_info(&self.guest_info).expect("extension info should be set");

        // Register the guest messages
        let guest = ExtensionGuest::new();
        listen_for_guest_message::<Pong>(app, &guest);
        listen_for_guest_message::<ShutdownGuest>(app, &guest);

        app.insert_resource(guest)
            .add_systems(PreUpdate, systems::send_guest_message_events);
    }
}

fn set_extension_info(extension_info: &ExtensionModuleInfo) -> Result<(), ExtensionError> {
    let encoded = rmp_serde::to_vec_named(extension_info)?;
    unsafe { etheryal_extension_sys::extension_info(encoded.len(), encoded.as_ptr()) };
    Ok(())
}

fn listen_for_guest_message<T>(app: &mut App, guest: &ExtensionGuest)
where
    T: GuestMessage,
    GuestMessageEnum: TryInto<T, Error = &'static str>, {
    let type_id = TypeId::of::<T>();

    assert!(
        !guest.guest_messages.contains_key(&type_id),
        "Duplicate registration of a guest Message: {}",
        type_name::<T>()
    );
    guest.guest_messages.insert(type_id, SegQueue::new());

    app.add_event::<ExtensionEvent<T>>()
        .add_systems(PreUpdate, send_message_event::<T>);
}

fn send_message_event<T>(guest: Res<ExtensionGuest>, mut events: EventWriter<ExtensionEvent<T>>)
where
    T: GuestMessage,
    GuestMessageEnum: TryInto<T, Error = &'static str>, {
    let Some(messages) = guest.guest_messages.get(&TypeId::of::<T>()) else {
        return;
    };

    while let Some(message) = messages.pop() {
        let Ok(inner) = message.try_into() else {
            warn!("Failed to downcast guest message to '{}'", type_name::<T>());
            continue;
        };
        debug!("Received an extension guest message: {}", type_name::<T>());
        events.send(ExtensionEvent::new(inner));
    }
}
