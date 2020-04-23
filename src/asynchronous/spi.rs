//! SPI bus support.
use core::fmt;
use core::pin;
use core::task;

pub mod begin_transaction;
pub mod complete;

/// A peripheral that supports performing SPI operations.
pub trait Spi: fmt::Debug {
    /// The type of error that can occur while performing SPI operations.
    type Error;
    /// The associated SPI transaction type.
    type Transaction: SpiTransaction<Error = Self::Error>;

    /// Initiates a SPI transaction.
    ///
    /// This usually involves pulling down a chip select pin when operating in master mode, or
    /// waiting for a chip select pin to be pulled low when operating in slave mode.
    fn poll_begin_transaction(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<Self::Transaction, Self::Error>>;
}

/// Utility methods for types implementing [`Spi`].
pub trait SpiExt: Spi {
    /// Initiates a SPI transaction.
    ///
    /// This usually involves pulling down a chip select pin when operating in master mode, or
    /// waiting for a chip select pin to be pulled low when operating in slave mode.
    fn begin_transaction(&mut self) -> begin_transaction::BeginTransaction<Self>
    where
        Self: Unpin,
    {
        begin_transaction::begin_transaction(self)
    }
}

impl<A> SpiExt for A where A: Spi {}

/// A SPI transaction, that allows for transferring data until the transaction is dropped.
pub trait SpiTransaction: fmt::Debug {
    /// The type of error that can occur while performing SPI operations.
    type Error;
    /// The type of transfer used for the `transfer` method.
    type Transfer: SpiTransfer<Error = Self::Error>;
    /// The type of transfer used for the `transfer_split` method.
    type TransferSplit: SpiTransfer<Error = Self::Error>;

    /// Initiates a new transfer of data where the same buffer is used both for sending and
    /// receiving data.
    fn transfer(&mut self, buffer: &mut [u8]) -> Result<Self::Transfer, Self::Error>;

    /// Initiates a new transfer with a separate sending and receiving buffer.
    fn transfer_split(
        &mut self,
        tx_buffer: &[u8],
        rx_buffer: &mut [u8],
    ) -> Result<Self::TransferSplit, Self::Error>;
}

/// A SPI transfer that is in progress.
pub trait SpiTransfer {
    /// The type of error that can occur while performing SPI operations.
    type Error;
    /// Complete this transfer, meaning that the transfer is completely done.
    fn poll_complete(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), Self::Error>>;
}

/// Utility methods for types implementing [`SpiTransfer`].
pub trait SpiTransferExt: SpiTransfer {
    /// Complete this transfer, meaning that the transfer is completely done.
    fn complete(&mut self) -> complete::Complete<Self>
    where
        Self: Unpin,
    {
        complete::complete(self)
    }
}

impl<T> SpiTransferExt for T where T: SpiTransfer {}
