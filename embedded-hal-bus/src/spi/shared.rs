use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus};

use crate::spi::DeviceError;

/// Common implementation to perform a transaction against the device.
#[inline]
pub fn transaction<Word, BUS, CS, D>(
    operations: &mut [Operation<Word>],
    bus: &mut BUS,
    delay: &mut D,
    cs: &mut CS,
    cs_to_clock_delay_ns: u32,
    clock_to_cs_delay_ns: u32,
) -> Result<(), DeviceError<BUS::Error, CS::Error>>
where
    BUS: SpiBus<Word> + ErrorType,
    CS: OutputPin,
    D: DelayNs,
    Word: Copy,
{
    cs.set_low().map_err(DeviceError::Cs)?;
    if cs_to_clock_delay_ns > 0 {
        delay.delay_ns(cs_to_clock_delay_ns);
    }

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
    if clock_to_cs_delay_ns > 0 {
        delay.delay_ns(cs_to_clock_delay_ns);
    }
    let cs_res = cs.set_high();

    op_res.map_err(DeviceError::Spi)?;
    flush_res.map_err(DeviceError::Spi)?;
    cs_res.map_err(DeviceError::Cs)?;

    Ok(())
}
