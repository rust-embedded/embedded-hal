//! CAN Bus HAL
//!
//! The intent of this HAL is to facilitate the creation of platform agnostic CAN drivers and
//! higher level protocols.  Please refer to the embedded-hal crate documentation for more details.

#[cfg(feature = "unproven")]
use nb;

#[cfg(feature = "unproven")]
/// A standard representation of the frames that might be sent and received on a CAN bus.
///
/// This struct can represent any CAN frame, as described in the CAN specification
/// version 2.0, published September 1991 by Bosch GmbH.  They can be used for either
/// transmission or reception.
pub struct CanFrame {
  /// This contains either the Base Identifier or the Extended Identifier, depending on `ide`.
  pub id: u32,
  /// Number of bytes in the payload.
  pub dlc: u8,
  /// The frame's data payload, only the first `dlc` bytes are valid.
  pub data: CanPayload,
  /// True iff this frame is a Remote Transmission Request.
  pub rtr: bool,
  /// True iff the id field is extended (ie 29 bits long, as opposed to 11).
  pub ide: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved0: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved1: bool,
}

#[cfg(feature = "unproven")]
#[repr(C)]
/// CanPayload is for convenience since multi-byte values are routinely sent in CAN messages.
///
/// CAN frames routinely contain values of all different sizes and alignments, so it helps to be
/// able to do the conversions ad hoc while minimizing boilerplate bit manipulation, enter this
/// enum.  Aligned values can be assigned naturally, and then on the driver end the registers
/// can be populated by word (and vice versa for receiving).
///
/// # Safety
/// Though extracting values from a union is unsafe in general, there is no unsafe way to access
/// this union on most platforms; all fields are the same size.  Worry not.
///
/// I say "on most
/// platforms" because most higher level protocols that rest on CAN (eg CANOpen and J1939) specify
/// little endian encoding for multi-byte values, which happens to be what most MCUs and CPUs use.
/// If you're using some weird protocol that encodes in big endian, or tunneling IP traffic over
/// a CAN bus you should only use data8 and do the value reassembly manually.
pub union CanPayload {
  /// Interpret the payload as an array of 8 bit values
  pub data8: [u8; 8],
  /// Interpret the payload as an array of 16 bit values
  pub data16: [u16; 4],
  /// Interpret the payload as an array of 32 bit values
  pub data32: [u32; 2],
  /// Interpret the payload as a single 64 bit value
  pub data64: u64,
}

#[cfg(feature = "unproven")]
/// A standard representation of the frames that might be sent and received on a CAN FD bus.
///
/// This struct can represent any CAN FD frame, as described in the CAN FD specification
/// version 1.0, published April 2012 by Bosch GmbH.  They can be used for either
/// transmission or reception.
pub struct CanFdFrame {
  /// This contains either the Base Identifier or the Extended Identifier, depending on `ide`.
  pub id: u32,
  /// Number of bytes in the payload.
  ///
  /// # Note
  /// This is *not* the DLC field value, this is the number of bytes of payload in the frame,
  /// in CAN FD those are not the same thing, but the implementation of this HAL should hide that
  /// from you.
  pub data_length: u8,
  /// The frame's data payload, only the first `data_length` bytes are valid.
  pub data: CanFdPayload,
  /// True iff the id field is extended (ie 29 bits long, as opposed to 11).
  pub ide: bool,
  /// True iff this is a CAN FD format frame.  Including it here to give implementations the option
  /// to use CanFdFrame for all traffic on the bus, if they so choose.
  pub edl: bool,
  /// True iff the frame was sent with a switched bit rate.
  pub brs: bool,
  /// True iff the sender is in FaultConfinementState::ErrorPassive (or possibly in BusOff).
  pub esi: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved0: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved1: bool,
}

#[cfg(feature = "unproven")]
#[repr(C)]
/// CanFdPayload is for convenience since multi-byte values are routinely sent in CAN FD messages.
///
/// CAN FD frames routinely contain values of all different sizes and alignments, so it helps to be
/// able to do the conversions ad hoc while minimizing boilerplate bit manipulation, enter this
/// enum.  Aligned values can be assigned naturally, and then on the driver end the registers
/// can be populated by word (and vice versa for receiving).
///
/// # Safety
/// Though extracting values from a union is unsafe in general, there is no unsafe way to access
/// this union on most platforms; all fields are the same size.  Worry not.
///
/// I say "on most
/// platforms" because most higher level protocols that rest on CAN (eg CANOpen and J1939) specify
/// little endian encoding for multi-byte values, which happens to be what most MCUs and CPUs use.
/// If you're using some weird protocol that encodes in big endian, or tunneling IP traffic over
/// a CAN bus you should only use data8 and do the value reassembly manually.
pub union CanFdPayload {
  /// Interpret the payload as an array of 8 bit values
  pub data8: [u8; 64],
  /// Interpret the payload as an array of 16 bit values
  pub data16: [u16; 32],
  /// Interpret the payload as an array of 32 bit values
  pub data32: [u32; 16],
  /// Interpret the payload as an array of 64 bit values
  pub data64: [u64; 8],
  // /// Interpret the payload as an array of 128 bit values
  // pub data128: [u128; 4],  // TODO: put me back in once u128 stabilizes, because why not
}

#[cfg(feature = "unproven")]
/// Converts a CAN-FD DLC into a byte count.
///
/// NOTE: According to the CAN 2.0 spec a data length of 8 can be encoded as any DLC >= 8.
/// This function has no way of knowing the frame type, so be sure to only call it after
/// you've verified that it's a CAN-FD frame you're dealing with.
pub fn can_fd_dlc_to_byte_count(dlc: u8) -> u8 {
  match dlc & 0xF {
    0...8 => dlc,
    9...12 => 8 + 4 * (dlc & 0b111),
    13 => 32,
    14 => 48,
    15 => 64,
    _ => unreachable!(),
  }
}

#[cfg(feature = "unproven")]
/// Converts a byte count into a CAN-FD DLC.
///
/// NOTE: Not all byte counts can be represented as DLCs, which by implication means that not all
/// byte counts are valid CAN-FD frame sizes.  This function accounts for the truncation and
/// padding that may be incurred as a result of that.
///
/// If n != byte_count_to_can_fd_dlc(can_fd_dlc_to_byte_count(n)) truncation or padding will occur.
pub fn byte_count_to_can_fd_dlc(byte_count: u8) -> u8 {
  match byte_count {
    0...8 => byte_count,
    9...12 => 0b1001,
    13...16 => 0b1010,
    17...20 => 0b1011,
    21...24 => 0b1100,
    25...32 => 0b1101,
    32...48 => 0b1110,
    _ => 0b1111,
  }
}

#[cfg(feature = "unproven")]
/// The intended behavior of a CAN filter.
pub enum MessageFilterType {
  /// Signifies a filter that includes traffic it matches.
  MatchMeansAccept,
  /// Signifies a filter that excludes traffic it matches.
  MatchMeansIgnore,
}

#[cfg(feature = "unproven")]
/// A filter that the hardware might apply to incomming traffic.
pub struct MessageFilter {
  /// The CAN id (or common subset of a CAN idea if a mask is specified) to filter or select.
  pub id: u32,
  /// Incomming CAN ids are masked with this mask (if present) before being compared against id.
  pub mask: Option<u32>,
  /// The intent of this filter, is it a "forward if" or a "forward unless"?
  pub filter_type: MessageFilterType,
}

#[cfg(feature = "unproven")]
/// The 3 fault confinement states as described in the CAN 2.0 spec.
pub enum FaultConfinementState {
  /// Errors are so few that this interface tells the whole bus when they happen.
  ErrorActive,
  /// Errors are numerous enough that informing the bus of them isn't allowed, but regular Rx
  /// and Tx can still work.
  ErrorPassive,
  /// There are so many bus errors that we're effectively not connected, Rx and Tx are disabled.
  BusOff,
}

// TODO: these should be constants, once uom supports no_std replace them with actual bitrates
#[cfg(feature = "unproven")]
/// An enumeration of all common CAN bitrates.
pub enum Bitrate {
  /// Run the bus at 10Kbps
  Nominal10Kbps,
  /// Run the bus at 20Kbps
  Nominal20Kbps,
  /// Run the bus at 50Kbps
  Nominal50Kbps,
  /// Run the bus at 125Kbps
  Nominal125Kbps,
  /// Run the bus at 250Kbps
  Nominal250Kbps,
  /// Run the bus at 500Kbps
  Nominal500Kbps,
  /// Run the bus at 800Kbps
  Nominal800Kbps,
  /// Run the bus at 1Mbps
  Nominal1Mbps,
}

// TODO: these should be constants, once uom supports no_std replace them with actual bitrates
#[cfg(feature = "unproven")]
/// An enumeration of common CAN-FD data phase bitrates.
pub enum FdDataBitrate {
  /// Run the data phase at 1Mbps
  Nominal1Mbps,
  // TODO: find out what the common settings are and put them here
  /// Run the data phase at 10Mbps
  Nominal10Mbps,
}

/// `TargetSamplePoint` exists to force users to pass sanity checked values to `set_speed`.
pub struct TargetSamplePoint {
  /// Decipercentage of the way through the bit that the sample point should be.
  tenths: u16,
}

impl TargetSamplePoint {
  /// Constructs a target sample point after some sanity checking.
  pub fn new(tenths_of_a_percent: u16) -> TargetSamplePoint {
    // TODO: add sanity checking here that the value is between 500 and 900
    TargetSamplePoint { tenths: tenths_of_a_percent }
  }

  /// Gets the target sample point in units of decipercent of a bit.
  pub fn value_as_tenths(&self) -> u16 { self.tenths }
}

/// CAN timing is controlled in units of Time Quanta, this codifies that.
pub struct QuantaCount(u8);

#[cfg(feature = "unproven")]
/// Operation Modes describe what the interface is currently doing.
pub enum InterfaceOperationMode {
  /// The interface is currently receiving a message from the bus.
  Receiver,
  /// The interface is currently transmitting a message from the bus.
  Transmitter,
  /// The interface is waiting to sync with the bus (detect 11 consecutive recessive bits).
  ///
  /// NOTE: this state was never described in the CAN 2.0 spec, only the CAN FD spec, so
  /// documentation not written with CAN FD in mind may not talk about how to detect it.
  /// That said, it is applicable to regular CAN hardware, they have this state for the same
  /// reason CAN-FD does, so people implementing `CanInterface` for non-FD hardware may have
  /// to do some thinking.
  Integrating,
  /// The interface ready and waiting to either transmit or receive.
  ///
  /// NOTE: this state was never described in the CAN 2.0 spec, only the CAN FD spec, so
  /// documentation not written with CAN FD in mind may not talk about how to detect it.
  /// That said, it is applicable to regular CAN hardware, they have this state for the same
  /// reason CAN-FD does, so people implementing `CanInterface` for non-FD hardware may have
  /// to do some thinking.
  Idle,
}

#[cfg(feature = "unproven")]
/// This trait represents a CAN interface, be it regular CAN or CAN-FD.
pub trait CanInterface<Frame> {
  /// Whatever error type the implementation finds useful.
  type Error;  // TODO: think about if this would be better as another type parameter.

  /// Returns true iff the hardware supports CAN-FD.
  fn supports_can_fd(&self) -> bool;

  /// Returns true iff the hardware supports ISO-CAN-FD (a fixed version of CAN-FD).
  fn supports_iso_can_fd(&self) -> bool;

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
  /// * `nominal_bitrate` - The bus-wide bitrate.
  /// * `sample_point` - The target point in the bit for the interface to sample at (in tenths of a percent).
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_speed(&mut self,
               nominal_bitrate: Bitrate,
               sample_point: TargetSamplePoint,
               jump_width: QuantaCount);

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
  /// The raw version of `set_speed` does no sanity checking to ensure that the provided
  /// segments correspond to a reasonable sample point or a reasonable bitrate, use with care.
  ///
  /// * `seg1` - The number of time quanta between the end of the propagation segment and the bit sample point.
  /// * `seg2` - The number of time quanta between the sample point and the end of the bit.
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_speed_raw(&mut self,
                   seg1: QuantaCount,
                   seg2: QuantaCount,
                   jump_width: QuantaCount);

  /// Sets the bit timing parameters for the data phase of CAN-FD frames.
  ///
  /// If neither this function nor `set_data_speed_raw` are ever called the implementation should
  /// default to the parameters provided for the arbitration phase.
  ///
  /// This function queries the system for (or just knows) the relevant clock frequency and then
  /// uses it and `nominal_bitrate` to compute seg1 and seg2 such that the bit is sampled at
  /// `sample_point`.  It then passes these computed values to `set_data_speed_raw`.
  ///
  /// * `nominal_bitrate` - The bus-wide data segment bitrate.
  /// * `sample_point` - The target point in the bit for the interface to sample at (in tenths of a percent).
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_data_speed(&mut self,
                    nominal_bitrate: FdDataBitrate,
                    sample_point: TargetSamplePoint,
                    jump_width: QuantaCount);

  /// Sets the bit timing parameters for the data phase of CAN-FD frames.
  ///
  /// If neither this function nor `set_data_speed` are ever called the implementation should
  /// default to the parameters provided for the arbitration phase.
  ///
  ///
  /// The raw version of `set_data_speed` does no sanity checking to ensure that the provided
  /// segments correspond to a reasonable sample point or a reasonable bitrate, use with care.
  ///
  /// * `seg1` - The number of time quanta between the end of the propagation segment and the bit sample point.
  /// * `seg2` - The number of time quanta between the sample point and the end of the bit.
  /// * `jump_width` - The number of time quanta by which the system adjusts seg1 and seg2 as it synchronizes with the bus (1 is a good default).
  fn set_data_speed_raw(&mut self,
                        seg1: QuantaCount,
                        seg2: QuantaCount,
                        jump_width: QuantaCount);

  /// Gets the curernt operation mode (as defined by CAN-FD but present in regular CAN).
  fn current_operation_mode(&self) -> InterfaceOperationMode;

  /// Returns true iff this interface configured not to send dominant bits.
  ///
  /// Bus Monitoring Mode is defined in CAN-FD, but some MCUs (notably STM-32s) support this
  /// feature even on a regular CAN bus, check your local documentation closely to see if you
  /// have such a mode (though it may have a different name), otherwise return false.
  fn in_bus_monitoring_mode(&self) -> bool;

  /// Returns true iff this interface is able to send and receive, but not send error or overload
  /// frames.
  ///
  /// Restricted Operation Mode is defined in CAN-FD, regular CAN implementations should just
  /// return false.
  fn in_restricted_operation_mode(&self) -> bool;

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

  // TODO: determine if we need the following, or if it can be accomplished via transmit
  // /// Tell some remote hardware to leave the sleep state.
  // fn send_wakeup(&self);

  /// Fault confinement states are used by CAN to control how errors are reported.
  ///
  /// This method should be overridden in implementations where the hardware gives access to
  /// the fault confinement state that it is using.
  fn fault_confinement_state(&self) -> FaultConfinementState {
    let tx_errors = self.transmit_error_count();
    if tx_errors >= 256 {
      FaultConfinementState::BusOff
    } else if tx_errors >= 128 || self.receive_error_count() >= 128 {
      FaultConfinementState::ErrorPassive
    } else {
      FaultConfinementState::ErrorActive
    }
  }

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

