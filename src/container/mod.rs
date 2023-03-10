pub mod code_section;
pub mod data_section;
pub mod type_section;

use crate::{version::Version, header::Header};
use code_section::CodeSection;
use data_section::DataSection;
use type_section::TypeSection;

/// EOF Validated Container
pub struct Container<'a> {
    /// EOF Version
    version: Version,
    /// EOF Header
    header: Header,
    /// EOF Type Section
    type_section: TypeSection,
    /// EOF Code Section
    code_section: CodeSection<'a>,
    /// EOF Data Section
    data_section: DataSection,
}

impl<'a> Container<'a> {
    // TODO:
    // pub fn new() -> Self {}

    /// Get type section
    pub fn type_section(&self) -> &TypeSection {
        &self.type_section
    }

    /// Get code section
    pub fn code_section(&self) -> &CodeSection {
        &self.code_section
    }

    /// Get data section
    pub fn data_section(&self) -> &DataSection {
        &self.data_section
    }
}
