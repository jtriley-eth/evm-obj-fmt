/// EOF Error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid Version
    Version,
    /// Invalid Header
    Header,
    /// OpCode Required Immediates Mismatch
    OpCodeImmediates,
    /// Invalid Bytecode Length
    BytecodeLength,
    /// Not an EOF Container
    NotEOF,
    /// Type Section Malformed
    TypeSectionMalformed,
    /// Number of Code Sections Mismatch with Type Section
    NumberOfCodeSections,
    /// Code Section Metadata Mismatch with Type Section
    CodeSectionMetadata,
    /// First Code Section MUST Have 0 Inputs and 0 Outputs
    FirstCodeSection,
    /// Invalid OpCode
    InvalidOpcode,
}
