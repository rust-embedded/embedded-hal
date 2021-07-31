//! v2 compatibility shims
//!
//! This module adds implicit forward support to v1 digital traits,
//! allowing v1 implementations to be directly used with v2 consumers.
//!
//! ```
//! extern crate embedded_hal;
//! use embedded_hal::digital::{v1, v2};
//!
//! struct OldOutputPinImpl { }
//!
//! impl v1::OutputPin for OldOutputPinImpl {
//!     fn set_low(&mut self) { }
//!     fn set_high(&mut self) { }
//! }
//!
//! struct NewOutputPinConsumer<T: v2::OutputPin> {
//!     _pin: T,
//! }
//!
//! impl <T>NewOutputPinConsumer<T>
//! where T: v2::OutputPin {
//!     pub fn new(pin: T) -> NewOutputPinConsumer<T> {
//!         NewOutputPinConsumer{ _pin: pin }
//!     }
//! }
//!
//! fn main() {
//!     let pin = OldOutputPinImpl{};
//!     let _consumer = NewOutputPinConsumer::new(pin);
//! }
//! ```
//!

#[allow(deprecated)]
use super::v1;
use super::v2;

/// Implementation of fallible `v2::OutputPin` for `v1::OutputPin` traits
#[allow(deprecated)]
impl<T> v2::OutputPin for T
where
    T: v1::OutputPin,
{
    // TODO: update to ! when never_type is stabilized
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_high())
    }
}

/// Implementation of fallible `v2::StatefulOutputPin` for `v1::StatefulOutputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::StatefulOutputPin for T
where
    T: v1::StatefulOutputPin + v1::OutputPin,
{
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }

    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::toggleable::Default for T where T: v1::toggleable::Default {}

/// Implementation of fallible `v2::InputPin` for `v1::InputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::InputPin for T
where
    T: v1::InputPin,
{
    // TODO: update to ! when never_type is stabilized
    type Error = ();

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }
}

#[cfg(test)]
mod tests {

    #[allow(deprecated)]
    use crate::digital::v1;
    use crate::digital::v2;

    #[allow(deprecated)]
    struct OldOutputPinImpl {
        state: bool,
    }

    #[allow(deprecated)]
    impl v1::OutputPin for OldOutputPinImpl {
        fn set_low(&mut self) {
            self.state = false;
        }
        fn set_high(&mut self) {
            self.state = true;
        }
    }

    #[allow(deprecated)]
    #[cfg(feature = "unproven")]
    impl v1::StatefulOutputPin for OldOutputPinImpl {
        fn is_set_low(&self) -> bool {
            self.state == false
        }

        fn is_set_high(&self) -> bool {
            self.state == true
        }
    }

    #[allow(deprecated)]
    #[cfg(feature = "unproven")]
    impl v1::toggleable::Default for OldOutputPinImpl {}

    struct NewOutputPinConsumer<T: v2::OutputPin> {
        _pin: T,
    }

    impl<T> NewOutputPinConsumer<T>
    where
        T: v2::OutputPin,
    {
        pub fn new(pin: T) -> NewOutputPinConsumer<T> {
            NewOutputPinConsumer { _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    struct NewToggleablePinConsumer<T: v2::ToggleableOutputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    impl<T> NewToggleablePinConsumer<T>
    where
        T: v2::ToggleableOutputPin,
    {
        pub fn new(pin: T) -> NewToggleablePinConsumer<T> {
            NewToggleablePinConsumer { _pin: pin }
        }
    }

    #[test]
    #[cfg(feature = "unproven")]
    fn v2_v1_toggleable_implicit() {
        let i = OldOutputPinImpl { state: false };
        let _c = NewToggleablePinConsumer::new(i);
    }

    #[test]
    fn v2_v1_output_implicit() {
        let i = OldOutputPinImpl { state: false };
        let _c = NewOutputPinConsumer::new(i);
    }

    #[test]
    fn v2_v1_output_state() {
        let mut o = OldOutputPinImpl { state: false };

        v2::OutputPin::set_high(&mut o).unwrap();
        assert_eq!(o.state, true);

        v2::OutputPin::set_low(&mut o).unwrap();
        assert_eq!(o.state, false);
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinImpl {
        state: bool,
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl v1::InputPin for OldInputPinImpl {
        fn is_low(&self) -> bool {
            !self.state
        }
        fn is_high(&self) -> bool {
            self.state
        }
    }

    #[cfg(feature = "unproven")]
    struct NewInputPinConsumer<T: v2::InputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    impl<T> NewInputPinConsumer<T>
    where
        T: v2::InputPin,
    {
        pub fn new(pin: T) -> NewInputPinConsumer<T> {
            NewInputPinConsumer { _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    #[cfg(feature = "unproven")]
    fn v2_v1_input_implicit() {
        let i = OldInputPinImpl { state: false };
        let _c = NewInputPinConsumer::new(i);
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v2_v1_input_state() {
        let mut i = OldInputPinImpl { state: false };

        assert_eq!(v2::InputPin::is_high(&mut i).unwrap(), false);
        assert_eq!(v2::InputPin::is_low(&mut i).unwrap(), true);
    }
}
