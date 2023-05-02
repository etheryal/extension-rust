//! This crate provides raw interfaces between the Etheryal extension guest and
//! host.
#![deny(missing_docs)]

#[link(wasm_import_module = "host")]
extern "C" {
    /// This function is called by the extension guest to send information about
    /// the extension to the host.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the encoded extension information.
    /// * `ptr` - The pointer to the encoded extension information.
    ///
    /// # Errors
    ///
    /// This function will result in an execution trap if the extension guest
    /// attempts to send extension information to the host more than once.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a buffer of length `len`.
    pub fn extension_info(len: usize, ptr: *const u8);

    /// This function is called by the extension guest to send a message to the
    /// host.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the message.
    /// * `ptr` - The pointer to the message.
    ///
    /// # Errors
    ///
    /// This function will result in an execution trap if the extension guest
    /// attempts to send a message to the host before sending extension
    /// information.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a buffer of length `len`.
    pub fn send_message(len: usize, ptr: *const u8);

    /// This function is called by the extension guest to receive a new message
    /// from the host. This clears the message buffer.
    ///
    /// # Returns
    ///
    /// The length of the encoded message.
    ///
    /// # Errors
    ///
    /// This function will result in an execution trap if the extension guest
    /// attempts to receive a message from the host before sending extension
    /// information.
    ///
    /// # Safety
    ///
    /// This should only be called on the main thread.
    pub fn recv_message() -> usize;

    /// This function is called by the extension guest to read the host message
    /// buffer into an user-provided buffer.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the buffer.
    /// * `ptr` - The pointer to the buffer.
    ///
    /// # Returns
    ///
    /// The amount of data read into the buffer.
    ///
    /// # Errors
    ///
    /// This function will result in an execution trap if the extension guest
    /// attempts to read the host message buffer before sending extension
    /// information.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a buffer of length `len`.
    pub fn read_message_buf(len: usize, ptr: *mut u8) -> usize;
}
