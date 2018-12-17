//! v1 compatibility shims
//! this module adds reverse support for v2 digital traits
//! only on of v1_compat or v2_compat may be included at a given time

#[allow(deprecated)]
use super::v1;
use super::v2;

/// Implementation of v1 OutputPin trait for v2 fallible output pins
#[allow(deprecated)]
impl <T, ERR> v1::OutputPin for T
where
    T: v2::OutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn set_low(&mut self) {
        self.set_low().unwrap()
    }

    fn set_high(&mut self) {
        self.set_high().unwrap()
    }
}

/// Implementation of v1 StatefulOutputPin trait for v2 fallible pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, ERR> v1::StatefulOutputPin for T 
where
    T: v2::StatefulOutputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn is_set_low(&self) -> bool {
        self.is_set_low().unwrap()
    }

    fn is_set_high(&self) -> bool {
        self.is_set_high().unwrap()
    }
}

/// Implementation of v0.2 InputPin trait for v0.3 fallible pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, ERR> v1::InputPin for T 
where
    T: v2::InputPin<Error=ERR>,
    ERR: core::fmt::Debug,
{
    fn is_low(&self) -> bool {
        self.is_low().unwrap()
    }

    fn is_high(&self) -> bool {
        self.is_high().unwrap()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
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
    where T: v1::OutputPin {
        pub fn new(pin: T) -> OldOutputPinConsumer<T> {
            OldOutputPinConsumer{ _pin: pin }
        }
    }

    #[test]
    fn new_old() {
        let i = NewOutputPinImpl{state: false};
        let _c = OldOutputPinConsumer::new(i);
    }

}