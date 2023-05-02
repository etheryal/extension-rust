use thiserror::Error;

/// An error that can occur when interacting with the extension host
#[derive(Error, Debug)]
pub enum ExtensionError {
    /// An error occurred while decoding a message
    #[error("Failed to encode message: {0}")]
    Encode(#[from] rmp_serde::encode::Error),

    /// An error occurred while decoding a message
    #[error("Failed to decode message: {0}")]
    Decode(#[from] rmp_serde::decode::Error),

    /// Unknown message type
    #[error("Unknown message type: {0}")]
    UnknownMessage(String),

    /// Failed to downcast a message
    #[error("Failed to downcast message")]
    Downcast,
}
