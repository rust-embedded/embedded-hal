//! CAN Bus HAL
//!
//! The intent of this HAL is to facilitate the creation of platform agnostic CAN drivers and
//! higher level protocols.  Please refer to the embedded-hal crate documentation for more details.

#[cfg(feature = "unproven")]
extern crate can_utils;

#[cfg(feature = "unproven")]
use can::can_utils::interface::{ FaultConfinementState, InterfaceOperationMode, MessageFilter };

#[cfg(feature = "unproven")]
use can::can_utils::timing_calculator::{ BitSamplePoint, BitsPerSecond, CanBitTimingParameters, MegaHertz, SegmentLength };

#[cfg(feature = "unproven")]
use nb;

#[cfg(feature = "unproven")]
/// Exactly what it sounds like, it's a HAL for CAN interfaces to allow generic drivers.
pub trait CanInterface : BaseCanInterface<can_utils::CanFrame> {}

#[cfg(feature = "unproven")]
/// Exactly what it sounds like, it's a HAL for CAN-FD interfaces to allow generic drivers.
pub trait CanFdInterface : BaseCanInterface<can_utils::CanFdFrame> {
  /// Returns true iff this interface is able to send and receive, but not send error or overload
  /// frames.
  fn in_restricted_operation_mode(&self) -> bool;

  /// Returns true iff the hardware supports CAN-FD.
  fn supports_can_fd(&self) -> bool;

  /// Returns true iff the hardware supports ISO-CAN-FD (a fixed version of CAN-FD).
  fn supports_iso_can_fd(&self) -> bool;
}

#[cfg(feature = "unproven")]
/// Declares the common functionalities between CAN interfaces and CAN-FD interfaces.
pub trait BaseCanInterface<Frame> {
  /// Whatever error type the implementation finds useful.
  type Error;  // TODO: think about if this would be better as another type parameter.

  /// Maybe populates buf with a frame from the bus, and returns if it did so.
  fn receive(&mut self, buf: &mut Frame) -> nb::Result<bool, Self::Error>;

  /// Sends a frame to all listeners on the bus.
  fn transmit(&mut self, frame: &Frame) -> nb::Result<(), Self::Error>;

  /// Sets the bit timing parameters for the bus (for CAN-FD these are the parameters for the arbitration phase).
  ///
  /// This function queries the system for (or just knows) the relevant clock frequency and then
  /// uses it and `nominal_bitrate` to compute seg1 and seg2 such that the bit is sampled at
  /// `sample_point`.  It then passes these computed values to `set_data_speed_raw`.
  ///
  /// If this is a CAN-FD interface and the data phase speed registers are at their reset values,
  /// implementations should set the data phase parameters to match the values provided here.
  ///
  /// Drivers should call either this function or `set_speed_raw`.
  ///
  /// Implementation hint: you can use can_utils::timing_calculator to convert
  /// these parameters to a CanBitTimingParameters
  ///
  /// * `nominal_bitrate` - The bus-wide bitrate, eg 250kbps.
  /// * `sample_point` - The target point in the bit for the interface to sample at (in tenths of a percent).
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_speed(&mut self,
               nominal_bitrate: BitsPerSecond,
               sample_point: BitSamplePoint,
               jump_width: SegmentLength);

  /// Sets the bit timing parameters for the bus (for CAN-FD these are the parameters for the arbitration phase).
  ///
  /// If this is a CAN-FD interface and the data phase speed registers are at their reset values,
  /// implementations should set the data phase parameters to match the values provided here.
  ///
  /// Drivers should call either this function or `set_speed`.  If this is a CAN-FD interface the
  /// driver should set the arbitration phase parameters before the data phase parameters, just in
  /// case the HAL implementation is doing something silly like resetting the data parameters when
  /// new arbitration parameters arrive.
  ///
  /// The raw version of `set_speed` might not sanity check to ensure that the provided
  /// segments correspond to a reasonable sample point or a reasonable bitrate, use with care.
  fn set_speed_raw(&mut self, timing_parameters: CanBitTimingParameters);

  /// Sets the bit timing parameters for the data phase of CAN-FD frames.
  ///
  /// If neither this function nor `set_data_speed_raw` are ever called the implementation must
  /// default to the parameters provided for the arbitration phase.
  ///
  /// This function queries the system for (or just knows) the relevant clock frequency and then
  /// uses it and `nominal_bitrate` to compute seg1 and seg2 such that the bit is sampled at
  /// `sample_point`.  It then passes these computed values to `set_data_speed_raw`.
  ///
  /// Implementation hint: you can use can_utils::timing_calculator to convert
  /// these parameters to a CanBitTimingParameters
  ///
  /// * `nominal_bitrate` - The bus-wide data segment bitrate, eg 250kbps.
  /// * `sample_point` - The target point in the bit for the interface to sample at (in tenths of a percent).
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_data_speed(&mut self,
                    nominal_bitrate: BitsPerSecond,
                    sample_point: BitSamplePoint,
                    jump_width: SegmentLength);

  /// Sets the bit timing parameters for the data phase of CAN-FD frames.
  ///
  /// If neither this function nor `set_data_speed` are ever called the implementation must
  /// default to the parameters provided for the arbitration phase.
  ///
  /// The raw version of `set_data_speed` might not sanity check to ensure that the provided
  /// segments correspond to a reasonable sample point or a reasonable bitrate, use with care.
  fn set_data_speed_raw(&mut self, timing_parameters: CanBitTimingParameters);

  /// Gets the speed of whichever clock controls the CAN interface.
  fn relevant_clock_speed(&self) -> MegaHertz;

  /// This returns the largest timing values the hardware can accept.
  fn maximum_timing_values(&self) -> CanBitTimingParameters;

  /// Gets the curernt operation mode (as defined by CAN-FD but present in regular CAN).
  fn current_operation_mode(&self) -> InterfaceOperationMode;

  /// Returns true iff this interface configured not to send dominant bits.
  ///
  /// Bus Monitoring Mode is defined in CAN-FD, but some MCUs (notably STM-32s) support this
  /// feature even on a regular CAN bus, check your local documentation closely to see if you
  /// have such a mode (though it may have a different name), otherwise return false.
  fn in_bus_monitoring_mode(&self) -> bool;

  /// Gets the number of unused slots in the hardware message filter bank.
  ///
  /// Zero here may mean either that there is no hardware filtering support on this platform, or
  /// that it exists but is full.  If you absolutely must determine if hardware filtering exists
  /// clear the filters and then call this method... but realistically you probably don't need
  /// to know that.
  fn unused_filter_bank_count(&self)-> u32;

  /// Adds an incomming message filter.
  fn add_filter(&mut self, filter: &MessageFilter);

  /// Removes a single incomming message filter, if it exists.
  fn remove_filter(&mut self, filter: &MessageFilter);

  /// Remove all incomming message filters.  After this call all valid traffic on the BUS is available
  /// via `receive`.
  fn clear_filters(&mut self);

  /// Returns true iff the CAN hardware is in the sleep state.
  fn is_asleep(&self) -> bool;

  /// Tell the hardware to enter the sleep state.
  fn request_sleep_mode(&mut self);

  /// Tell the hardware to leave the sleep state.
  fn request_wakeup(&mut self);

  /// Fault confinement states are used by CAN to control how errors are reported.
  ///
  /// Implementation hint: if your hardware doesn't give you this information you can use
  /// can_utils::interface::FaultConfinementState::from_error_counts to infer it.
  fn fault_confinement_state(&self) -> FaultConfinementState;

  /// Gets the receive error count.
  ///
  /// The exact rules for the meaning of the receive error count are too hairy to go into here,
  /// if you care what this means I'd encourage you to begin reading at page 24 of the CAN 2.0 spec.
  fn receive_error_count(&self) -> u32;

  /// Gets the transmit error count.
  ///
  /// The exact rules for the meaning of the transmit error count are too hairy to go into here,
  /// if you care what this means I'd encourage you to begin reading at page 24 of the CAN 2.0 spec.
  fn transmit_error_count(&self) -> u32;
}

