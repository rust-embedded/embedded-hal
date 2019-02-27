//! v1 compatibility wrapper
//! this module adds reverse support for v2 digital traits
//! v2 traits must be explicitly cast to the v1 version using `.into()`.

#[allow(deprecated)]
use super::v1;
use super::v2;

/// Wrapper to allow v2 fallible OutputPin traits to be converted to v1 traits
pub struct OldOutputPin<T> {
    pin: T,
}

impl <T, ERR> OldOutputPin<T>
where
    T: v2::OutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    /// Create a new OldOutputPin wrapper around a v2::OutputPin
    pub fn new(pin: T) -> Self {
        Self{pin}
    }
}

impl <T, ERR> From<T> for OldOutputPin<T>
where
    T: v2::OutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn from(pin: T) -> Self {
        OldOutputPin{pin}
    }
}

/// Implementation of v1 OutputPin trait for v2 fallible output pins
#[allow(deprecated)]
impl <T, ERR> v1::OutputPin for OldOutputPin<T>
where
    T: v2::OutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn set_low(&mut self) {
        self.pin.set_low().unwrap()
    }

    fn set_high(&mut self) {
        self.pin.set_high().unwrap()
    }
}

/// Implementation of v1 StatefulOutputPin trait for v2 fallible pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, ERR> v1::StatefulOutputPin for OldOutputPin<T> 
where
    T: v2::StatefulOutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn is_set_low(&self) -> bool {
        self.pin.is_set_low().unwrap()
    }

    fn is_set_high(&self) -> bool {
        self.pin.is_set_high().unwrap()
    }
}

/// Wrapper to allow v2 fallible InputPin traits to be converted to v1 traits
#[cfg(feature = "unproven")]
pub struct OldInputPin<T> {
    pin: T,
}

#[cfg(feature = "unproven")]
impl <T, ERR> OldInputPin<T>
where
    T: v2::OutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    /// Create an OldInputPin wrapper around a v2::InputPin
    pub fn new(pin: T) -> Self {
        Self{pin}
    }
}

#[cfg(feature = "unproven")]
impl <T, ERR> From<T> for OldInputPin<T>
where
    T: v2::InputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn from(pin: T) -> Self {
        OldInputPin{pin}
    }
}

/// Implementation of v0.2 InputPin trait for v0.3 fallible pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, ERR> v1::InputPin for OldInputPin<T>
where
    T: v2::InputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn is_low(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn is_high(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    use crate::digital::v1;
    use crate::digital::v2;

    struct NewOutputPinImpl { 
        state: bool
    }

    impl v2::OutputPin for NewOutputPinImpl {
        type Error = ();

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.state = false;
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), Self::Error>{
            self.state = true;
            Ok(())
        }
    }

    #[allow(deprecated)]
    struct OldOutputPinConsumer<T: v1::OutputPin> {
        _pin: T,
    }

    #[allow(deprecated)]
    impl <T>OldOutputPinConsumer<T> 
    where T: v1::OutputPin 
    {
        pub fn new(pin: T) -> OldOutputPinConsumer<T> {
            OldOutputPinConsumer{ _pin: pin }
        }
    }

    #[test]
    fn v1_v2_output_explicit() {
        let i = NewOutputPinImpl{state: false};
        let _c: OldOutputPinConsumer<OldOutputPin<_>> = OldOutputPinConsumer::new(i.into());
    }

    #[cfg(feature = "unproven")]
    struct NewInputPinImpl {
        state: bool,
    }

    #[cfg(feature = "unproven")]
    impl v2::InputPin for NewInputPinImpl {
        type Error = ();

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.state)
        }
        fn is_high(&self) -> Result<bool, Self::Error>{
            Ok(self.state)
        }
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinConsumer<T: v1::InputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl <T>OldInputPinConsumer<T> 
    where T: v1::InputPin 
    {
        pub fn new(pin: T) -> OldInputPinConsumer<T> {
            OldInputPinConsumer{ _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v1_v2_input_explicit() {
        let i = NewInputPinImpl{state: false};
        let _c: OldInputPinConsumer<OldInputPin<_>> = OldInputPinConsumer::new(i.into());
    }

}