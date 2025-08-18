/// Represents the response mode of the SD/MMC protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseMode {
    /// Standard SD mode of operation.
    Sd,
    /// SDIO mode of operation.
    Sdio,
    /// SPI mode of operation.
    Spi,
}
