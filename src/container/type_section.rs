use super::code_section::Function;

/// EOF Type Section
///
/// The type section currently only contains function metadata outlined in EIP-4750.
///
/// https://eips.ethereum.org/EIPS/eip-4750
pub struct TypeSection(Vec<FunctionMetadata>);

/// Function metadata packed into a single u32.
///
/// | u8    | u8     | u16              |
/// | ----- | ------ | ---------------- |
/// | input | output | max_stack_height |
pub type FunctionMetadata = u32;

impl FunctionMetadata {
    pub fn input(self) -> u8 {
        (self >> 24) as u8
    }

    pub fn output(self) -> u8 {
        ((self >> 16) & 0xff) as u8
    }

    pub fn max_stack_height(self) -> u16 {
        (self & 0xffff) as u16
    }
}

impl<'a> From<&Function<'a>> for FunctionMetadata {
    fn from(value: &Function) -> Self {
        (value.inputs() as u32) << 24
            | (value.outputs() as u32) << 16
            | (value.max_stack_height() as u32)
    }
}

impl From<&[u8]> for FunctionMetadata {
    fn from(value: &[u8]) -> Self {
        // accounts for endianness on the `max_stack_height`
        (value[0] as u32) << 24
            | ((value[1] as u32) << 16)
            | u32::from_be_bytes([value[2], value[3]])
    }
}
