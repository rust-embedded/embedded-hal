//! Blocking CAN API

/// A blocking CAN interface that is able to transmit and receive frames.
pub trait Can {
    /// Associated frame type.
    type Frame: crate::can::Frame;

    /// Associated error type.
    type Error;

    /// Puts a frame in the transmit buffer. Blocks until space is available in
    /// the transmit buffer.
    fn try_transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;

    /// Blocks until a frame was received or an error occured.
    fn try_receive(&mut self) -> Result<Self::Frame, Self::Error>;
}

/// Default implementation of `blocking::can::Can` for implementers of `can::Can`
pub trait Default: crate::can::Can {}

impl<S> crate::blocking::can::Can for S
where
    S: Default,
{
    type Frame = S::Frame;
    type Error = S::Error;

    fn try_transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        let mut replaced_frame;
        let mut frame_to_transmit = frame;
        while let Some(f) = nb::block!(self.try_transmit(&frame_to_transmit))? {
            replaced_frame = f;
            frame_to_transmit = &replaced_frame;
        }
        Ok(())
    }

    fn try_receive(&mut self) -> Result<Self::Frame, Self::Error> {
        nb::block!(self.try_receive())
    }
}
