//! v2 compatibility shims
//! this module adds implicit forward support to v1 digital traits

#[allow(deprecated)]
use super::v1;
use super::v2;

/// Implementation of v2 fallible OutputPin for v1 traits
#[allow(deprecated)]
impl <T> v2::OutputPin for T 
where
    T: v1::OutputPin,
{
    type Error = ();

    /// Toggle pin output
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }

     fn set_high(&mut self) -> Result<(), Self::Error> {
         Ok(self.set_high())
     }
}

/// Implementation of v2 fallible StatefulOutputPin for v1 digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> v2::StatefulOutputPin for T
where
    T: v1::StatefulOutputPin + v1::OutputPin,
{
    /// Toggle pin output
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }

     fn is_set_high(&self) -> Result<bool, Self::Error> {
         Ok(self.is_set_high())
     }
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> v2::InputPin for T
where
    T: v1::InputPin
{
    type Error = ();

    /// Toggle pin output
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
        state: bool
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

    struct NewOutputPinConsumer<T: v2::OutputPin> {
        _pin: T,
    }

    impl <T>NewOutputPinConsumer<T> 
    where T: v2::OutputPin {
        pub fn new(pin: T) -> NewOutputPinConsumer<T> {
            NewOutputPinConsumer{ _pin: pin }
        }
    }

    #[test]
    fn v2_v1_output_implicit() {
        let i = OldOutputPinImpl{state: false};
        let _c = NewOutputPinConsumer::new(i);
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinImpl { 
        state: bool
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
    impl <T>NewInputPinConsumer<T> 
    where T: v2::InputPin {
        pub fn new(pin: T) -> NewInputPinConsumer<T> {
            NewInputPinConsumer{ _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v2_v1_input_implicit() {
        let i = OldOutputPinImpl{state: false};
        let _c = NewOutputPinConsumer::new(i);
    }

}