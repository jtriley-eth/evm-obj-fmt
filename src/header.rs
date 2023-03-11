use crate::{error::Error, version::Version};

/// EOF Version Index
const VERSION_INDEX: usize = 2;

/// Type Section Size Index (upper byte)
const TYPE_SECTION_SIZE_INDEX_0: usize = 4;

/// Type Section Size Index (lower byte)
const TYPE_SECTION_SIZE_INDEX_1: usize = 5;

/// Number of Code Sections Index (upper byte)
const NUMBER_OF_CODE_SECTIONS_INDEX_0: usize = 7;

/// Number of Code Sections Index (lower byte)
const NUMBER_OF_CODE_SECTIONS_INDEX_1: usize = 8;

/// Code Section Size Index (upper byte)
const CODE_SECTION_SIZE_INDEX_0: usize = 9;

/// Code Section Size Index (lower byte)
const CODE_SECTION_SIZE_INDEX_1: usize = 10;

/// Data Section Size Index (upper byte)
const DATA_SECTION_SIZE_INDEX_0: usize = 12;

/// Data Section Size Index (lower byte)
const DATA_SECTION_SIZE_INDEX_1: usize = 13;

/// Header Size
const HEADER_SIZE: usize = 15;

/// Magic Byte (upper)
const MAGIC_0: u8 = 0xef;

/// Magic Byte (lower)
const MAGIC_1: u8 = 0x00;

/// Type Section Marker
const TYPE_SECTION_MARKER: u8 = 0x01;

/// Code Section Marker
const CODE_SECTION_MARKER: u8 = 0x02;

/// Data Section Marker
const DATA_SECTION_MARKER: u8 = 0x03;

/// Terminator
const TERMINATOR: u8 = 0x00;

/// Raw Header
pub type RawHeader = [u8; HEADER_SIZE];

/// Default Header (EOF Version 1)
pub const DEFAULT_HEADER: RawHeader = [
    // Magic Bytes
    MAGIC_0,
    MAGIC_1,
    // EOF Version
    0x00,
    // Type Section Marker
    TYPE_SECTION_MARKER,
    // Type Section Size
    0x00,
    0x00,
    // Code Section Marker
    CODE_SECTION_MARKER,
    // Number of Code Sections
    0x00,
    0x00,
    // Code Section Size
    0x00,
    0x00,
    // Data Section Marker
    DATA_SECTION_MARKER,
    // Data Section Size
    0x00,
    0x00,
    // Terminator
    TERMINATOR,
];

/// EOF Header
pub struct Header {
    raw: RawHeader,
}

impl Header {
    /// Returns the raw header
    pub fn raw(&self) -> RawHeader {
        self.raw
    }

    /// Returns Total Code Size
    pub fn codesize(&self) -> u64 {
        HEADER_SIZE as u64
            + self.type_section_size() as u64
            + self.code_section_size() as u64
            + self.data_section_size() as u64
    }

    /// Returns the version
    pub fn version(&self) -> Version {
        Version::try_from(self.raw[VERSION_INDEX])
            .expect("invalid version (this should never happen")
    }

    /// Returns the type section size
    pub fn type_section_size(&self) -> u16 {
        let upper_bits = (self.raw[TYPE_SECTION_SIZE_INDEX_0] as u16) << 8;
        upper_bits | self.raw[TYPE_SECTION_SIZE_INDEX_1] as u16
    }

    /// Returns the number of code sections
    pub fn number_of_code_sections(&self) -> u16 {
        let upper_bits = (self.raw[NUMBER_OF_CODE_SECTIONS_INDEX_0] as u16) << 8;
        upper_bits | self.raw[NUMBER_OF_CODE_SECTIONS_INDEX_1] as u16
    }

    /// Returns the code section size
    pub fn code_section_size(&self) -> u16 {
        let upper_bits = (self.raw[CODE_SECTION_SIZE_INDEX_0] as u16) << 8;
        upper_bits | self.raw[CODE_SECTION_SIZE_INDEX_1] as u16
    }

    /// Returns the data section size
    pub fn data_section_size(&self) -> u16 {
        let upper_bits = (self.raw[DATA_SECTION_SIZE_INDEX_0] as u16) << 8;
        upper_bits | self.raw[DATA_SECTION_SIZE_INDEX_1] as u16
    }
}

pub struct HeaderBuilder {
    header: Header,
}

impl HeaderBuilder {
    /// Creates a new HeaderBuilder
    pub fn new() -> Self {
        Self {
            header: Header {
                raw: DEFAULT_HEADER,
            },
        }
    }

    /// Sets the version
    pub fn version(mut self, version: Version) -> Self {
        self.header.raw[VERSION_INDEX] = version.into();
        self
    }

    /// Sets the type section size
    pub fn type_section_size(mut self, size: u16) -> Self {
        self.header.raw[TYPE_SECTION_SIZE_INDEX_0] = (size >> 8) as u8;
        self.header.raw[TYPE_SECTION_SIZE_INDEX_1] = size as u8;
        self
    }

    /// Sets the number of code sections
    pub fn number_of_code_sections(mut self, number: u16) -> Self {
        self.header.raw[NUMBER_OF_CODE_SECTIONS_INDEX_0] = (number >> 8) as u8;
        self.header.raw[NUMBER_OF_CODE_SECTIONS_INDEX_1] = number as u8;
        self
    }

    /// Sets the code section size
    pub fn code_section_size(mut self, size: u16) -> Self {
        self.header.raw[CODE_SECTION_SIZE_INDEX_0] = (size >> 8) as u8;
        self.header.raw[CODE_SECTION_SIZE_INDEX_1] = size as u8;
        self
    }

    /// Sets the data section size
    pub fn data_section_size(mut self, size: u16) -> Self {
        self.header.raw[DATA_SECTION_SIZE_INDEX_0] = (size >> 8) as u8;
        self.header.raw[DATA_SECTION_SIZE_INDEX_1] = size as u8;
        self
    }

    /// Builds the Header
    pub fn build(self) -> Header {
        self.header
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_header() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0102)
            .number_of_code_sections(0x0001)
            .code_section_size(0x0304)
            .data_section_size(0x0506)
            .build();

        assert_eq!(header.version(), Version::V1);
        assert_eq!(header.type_section_size(), 0x0102);
        assert_eq!(header.number_of_code_sections(), 0x0001);
        assert_eq!(header.code_section_size(), 0x0304);
        assert_eq!(header.data_section_size(), 0x0506);
    }

    #[test]
    fn test_header_codesize() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .build();

        assert_eq!(header.codesize(), 15);
    }

    #[test]
    fn test_header_codesize_with_type_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 16);
    }

    #[test]
    fn test_header_codesize_with_code_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .number_of_code_sections(0x0001)
            .code_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 16);
    }

    #[test]
    fn test_header_codesize_with_data_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0000)
            .number_of_code_sections(0x0000)
            .code_section_size(0x0000)
            .data_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 16);
    }

    #[test]
    fn test_header_codesize_with_type_and_code_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0001)
            .number_of_code_sections(0x0001)
            .code_section_size(0x0001)
            .data_section_size(0x0000)
            .build();

        assert_eq!(header.codesize(), 17);
    }

    #[test]
    fn test_header_codesize_with_type_and_data_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0001)
            .number_of_code_sections(0x0000)
            .code_section_size(0x0000)
            .data_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 17);
    }

    #[test]
    fn test_header_codesize_with_code_and_data_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0000)
            .number_of_code_sections(0x0001)
            .code_section_size(0x0001)
            .data_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 17);
    }

    #[test]
    fn test_header_codesize_with_type_code_and_data_section() {
        let header = HeaderBuilder::new()
            .version(Version::V1)
            .type_section_size(0x0001)
            .number_of_code_sections(0x0001)
            .code_section_size(0x0001)
            .data_section_size(0x0001)
            .build();

        assert_eq!(header.codesize(), 18);
    }
}
