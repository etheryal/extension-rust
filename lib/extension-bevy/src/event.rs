use bevy_ecs::prelude::*;
use derive_more::Deref;
use etheryal_extension_common::message::GuestMessage;

/// An event that occurs in an extension
#[derive(Debug, Deref, Event)]
pub struct ExtensionEvent<T>
where
    T: GuestMessage, {
    #[deref]
    inner: T,
}

impl<T> ExtensionEvent<T>
where
    T: GuestMessage,
{
    pub(crate) const fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Get the inner data out of it
    pub fn into_inner(self) -> T {
        self.inner
    }
}
