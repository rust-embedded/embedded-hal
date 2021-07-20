//! Quadrature encoder interface traits

pub mod blocking;

/// Count direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// 3, 2, 1
    Downcounting,
    /// 1, 2, 3
    Upcounting,
}
