//! Analog-digital conversion traits

pub use super::Channel;

/// ADCs that sample on single channels per request, and do so at the time of the request.
///
/// This trait is the interface to an ADC that is configured to read a specific channel at the time
/// of the request (in contrast to continuous asynchronous sampling).
///
/// ```
/// use embedded_hal::adc::nb::{Channel, OneShot};
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
///    fn read(&mut self, pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
///        let chan = 1 << pin.channel();
///        self.power_up();
///        let result = self.do_conversion(chan);
///        self.power_down();
///        Ok(result.into())
///    }
/// }
/// ```
pub trait OneShot<ADC, Word, Pin: Channel<ADC>> {
    /// Error type returned by ADC methods
    type Error;

    /// Request that the ADC begin a conversion on the specified pin
    ///
    /// This method takes a `Pin` reference, as it is expected that the ADC will be able to sample
    /// whatever channel underlies the pin.
    fn read(&mut self, pin: &mut Pin) -> nb::Result<Word, Self::Error>;
}
