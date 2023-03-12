/// EOF Error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid Version
    Version,
    /// Invalid Raw Header
    Header,
    /// OpCode Required Immediates Mismatch
    OpCodeImmediates,
    /// Invalid Bytecode Length
    BytecodeLength,
    /// Not an EOF Container
    NotEOF,
}
