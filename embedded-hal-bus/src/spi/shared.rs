use core::convert::Infallible;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus};

/// Common implementation to perform a transaction against the device.
#[inline]
pub fn transaction<Word, BUS, CS, D>(
    operations: &mut [Operation<Word>],
    bus: &mut BUS,
    delay: &mut D,
    cs: &mut CS,
) -> Result<(), BUS::Error>
where
    BUS: SpiBus<Word> + ErrorType,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
    Word: Copy,
{
    into_ok(cs.set_low());

    let op_res = operations.iter_mut().try_for_each(|op| match op {
        Operation::Read(buf) => bus.read(buf),
        Operation::Write(buf) => bus.write(buf),
        Operation::Transfer(read, write) => bus.transfer(read, write),
        Operation::TransferInPlace(buf) => bus.transfer_in_place(buf),
        Operation::DelayNs(ns) => {
            bus.flush()?;
            delay.delay_ns(*ns);
            Ok(())
        }
    });

    // On failure, it's important to still flush and deassert CS.
    let flush_res = bus.flush();
    into_ok(cs.set_high());

    op_res.and(flush_res)
}

/// see https://github.com/rust-lang/rust/issues/61695
pub(crate) fn into_ok<T>(res: Result<T, Infallible>) -> T {
    match res {
        Ok(t) => t,
        Err(infallible) => match infallible {},
    }
}
