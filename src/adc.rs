//! Analog-digital conversion traits

#[cfg(feature = "unproven")]
use nb;

/// A marker trait to identify MCU pins that can be used as inputs to an ADC channel.
///
/// This marker trait denotes an object, i.e. a GPIO pin, that is ready for use as an input to the
/// ADC. As ADCs channels can be supplied by multiple pins, this trait defines the relationship
/// between the physical interface and the ADC sampling buffer.
///
/// ```
/// # use std::marker::PhantomData;
/// # use embedded_hal::adc::Channel;
///
/// struct Adc1; // Example ADC with single bank of 8 channels
/// struct Gpio1Pin1<MODE>(PhantomData<MODE>);
/// struct Analog(()); // marker type to denote a pin in "analog" mode
///
/// // GPIO 1 pin 1 can supply an ADC channel when it is configured in Analog mode
/// impl Channel<Adc1> for Gpio1Pin1<Analog> {
///     type ID = u8; // ADC channels are identified numerically
///
///     fn channel() -> u8 { 7_u8 } // GPIO pin 1 is connected to ADC channel 7
/// }
///
/// struct Adc2; // ADC with two banks of 16 channels
/// struct Gpio2PinA<MODE>(PhantomData<MODE>);
/// struct AltFun(()); // marker type to denote some alternate function mode for the pin
///
/// // GPIO 2 pin A can supply an ADC channel when it's configured in some alternate function mode
/// impl Channel<Adc2> for Gpio2PinA<AltFun> {
///     type ID = (u8, u8); // ADC channels are identified by bank number and channel number
///
///     fn channel() -> (u8, u8) { (0, 3) } // bank 0 channel 3
/// }
/// ```
#[cfg(feature = "unproven")]
pub trait Channel<ADC> {
    /// Channel ID type
    ///
    /// A type used to identify this ADC channel. For example, if the ADC has eight channels, this
    /// might be a `u8`. If the ADC has multiple banks of channels, it could be a tuple, like
    /// `(u8: bank_id, u8: channel_id)`.
    type ID;

    /// Get the specific ID that identifies this channel, for example `0_u8` for the first ADC
    /// channel, if Self::ID is u8.
    fn channel() -> Self::ID;

    // `channel` is a function due to [this reported
    // issue](https://github.com/rust-lang/rust/issues/54973). Something about blanket impls
    // combined with `type ID; const CHANNEL: Self::ID;` causes problems.
    //const CHANNEL: Self::ID;
}

/// ADCs that sample on single channels per request, and do so at the time of the request.
///
/// This trait is the interface to an ADC that is configured to read a specific channel at the time
/// of the request (in contrast to continuous asynchronous sampling).
///
/// ```
/// use embedded_hal::adc::{Channel, OneShot};
///
/// struct MyAdc; // 10-bit ADC, with 5 channels
/// # impl MyAdc {
/// #     pub fn power_up(&mut self) {}
/// #     pub fn power_down(&mut self) {}
/// #     pub fn do_conversion(&mut self, chan: u8) -> u16 { 0xAA55_u16 }
/// # }
///
/// impl<WORD, PIN> OneShot<MyAdc, WORD, PIN> for MyAdc
/// where
///    WORD: From<u16>,
///    PIN: Channel<MyAdc, ID=u8>,
/// {
///    type Error = ();
///
///    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
///        let chan = 1 << PIN::channel();
///        self.power_up();
///        let result = self.do_conversion(chan);
///        self.power_down();
///        Ok(result.into())
///    }
/// }
/// ```
#[cfg(feature = "unproven")]
pub trait OneShot<ADC, Word, Pin: Channel<ADC>> {
    /// Error type returned by ADC methods
    type Error;

    /// Request that the ADC begin a conversion on the specified pin
    ///
    /// This method takes a `Pin` reference, as it is expected that the ADC will be able to sample
    /// whatever channel underlies the pin.
    fn read(&mut self, pin: &mut Pin) -> nb::Result<Word, Self::Error>;
}
