//! DMA Interface

/// Static alias for DMA
pub type Static<T> = &'static mut T;

/// DMA Errors
#[derive(Debug)]
pub enum Error {
    /// Previous data got overwritten before it could be read because it was
    /// not accessed in a timely fashion
    Overrun,
    /// Transfer error
    Transfer,
}


/// DMA Transfer future
pub trait Transfer {
    /// Return type
    type Item: ?Sized;
    /// Return type
    type Payload;

    /// Get buffer
    fn deref(&self) -> &Self::Item;

    /// Check completion
    fn is_done(&self) -> Result<bool, Error>;

    /// Block
    fn wait(self) -> Result<(Static<Self::Item>, Self::Payload), Error>
    where
        Self::Item: Sized;
}
