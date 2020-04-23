//! Re-exports of all of the async extension traits.
pub use crate::asynchronous::gpio::InputPinExt as _;
pub use crate::asynchronous::gpio::IntoFloatingInputPin as _;
pub use crate::asynchronous::gpio::IntoOpenDrainOutputPin as _;
pub use crate::asynchronous::gpio::IntoPullDownInputPin as _;
pub use crate::asynchronous::gpio::IntoPullUpInputPin as _;
pub use crate::asynchronous::gpio::IntoPushPullOutputPin as _;
pub use crate::asynchronous::gpio::OutputPinExt as _;
pub use crate::asynchronous::i2c::I2cBusMappingExt as _;
pub use crate::asynchronous::i2c::I2cReadExt as _;
pub use crate::asynchronous::i2c::I2cWriteExt as _;
pub use crate::asynchronous::io::ReadExt as _;
pub use crate::asynchronous::io::WriteExt as _;
pub use crate::asynchronous::timer::IntoOneshotTimer as _;
pub use crate::asynchronous::timer::IntoPeriodicTimer as _;
pub use crate::asynchronous::timer::TimerExt as _;
