/// Represents SD/MMC command types.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandType {
    /// Addressed commands: point-to-point, no data transfer on DAT.
    Ac = 0,
    /// Addressed commands: point-to-point, data transfer on DAT.
    Adtc = 1,
    /// Broadcast commands no response. Only available if all CMD lines connected.
    Bc = 2,
    /// Broadcast commands with response. Only available if all CMD lines separated.
    Bcr = 3,
}
