//! Contains all the messages that can be sent between the extension host and
//! guest
use std::any::TypeId;

use debug::*;
use enum_dispatch::enum_dispatch;
use events::*;
use serde::{Deserialize, Serialize};

pub mod debug;
pub mod events;

/// A marker trait to signal that this message should be sent *to* the extension
/// host
#[enum_dispatch]
pub trait HostMessage: Send + Sync + 'static {
    /// Returns the name of the type of this message
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Returns the type id of this message
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// A marker trait to signal that this message should be sent *to* the extension
/// guest
#[enum_dispatch]
pub trait GuestMessage: Send + Sync + 'static {
    /// Returns the name of the type of this message
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Returns the type id of this message
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// An enum that contains all possible messages that can be sent *to* the
/// extension host
#[enum_dispatch(HostMessage)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum HostMessageEnum {
    ShutdownHost,
    Ping,
}

/// An enum that contains all possible messages that can be sent *to* the
/// extension guest
#[enum_dispatch(GuestMessage)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum GuestMessageEnum {
    ShutdownGuest,
    Pong,
}
