use super::code_section::Function;

/// EOF Type Section
///
/// The type section currently only contains function metadata outlined in EIP-4750.
///
/// https://eips.ethereum.org/EIPS/eip-4750
pub struct TypeSection {
    /// Types
    types: Vec<FunctionMetadata>,
}

impl TypeSection {
    /// Create type section from reference to functions.
    pub fn new(functions: &[Function]) -> Self {
        Self {
            types: functions
                .iter()
                .map(|fun| fun.into())
                .collect(),
        }
    }
}

/// Function metadata packed into a single u32.
///
/// | u8    | u8     | u16              |
/// | ----- | ------ | ---------------- |
/// | input | output | max_stack_height |
pub type FunctionMetadata = u32;

impl<'a> From<&Function<'a>> for FunctionMetadata {
    fn from(value: &Function) -> Self {
        (value.inputs() as u32) << 24
            | (value.outputs() as u32) << 16
            | (value.max_stack_height() as u32)
    }
}
