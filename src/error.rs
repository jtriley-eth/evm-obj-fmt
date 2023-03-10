/// EOF Error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid Version
    InvalidVersion,
    /// Invalid Raw Header
    InvalidHeader,
    /// OpCode Required Immediates Mismatch
    OpCodeImmediates,
}
