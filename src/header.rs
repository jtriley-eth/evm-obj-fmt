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
    /// EOF Version
    version: u8,
    /// Type Section Size
    type_section_size: u16,
    /// Number of Code Sections
    number_of_code_sections: u16,
    /// Code Section Size
    code_section_size: u16,
    /// Data Section Size
    data_section_size: u16,
}

impl Header {
    /// Returns Total Code Size
    pub fn codesize(&self) -> usize {
        HEADER_SIZE
            + self.type_section_size as usize
            + self.code_section_size as usize
            + self.data_section_size as usize
    }

    /// Returns the version
    pub fn version(&self) -> Result<Version, Error> {
        Version::try_from(self.version)
    }

    /// Returns the type section size
    pub fn type_section_size(&self) -> u16 {
        self.type_section_size
    }

    /// Returns the number of code sections
    pub fn number_of_code_sections(&self) -> u16 {
        self.number_of_code_sections
    }

    /// Returns the code section size
    pub fn code_section_size(&self) -> u16 {
        self.code_section_size
    }

    /// Returns the data section size
    pub fn data_section_size(&self) -> u16 {
        self.data_section_size
    }
}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(value: RawHeader) -> Result<Self, Self::Error> {
        let _ = Version::try_from(value[VERSION_INDEX])?;

        let type_section_size: u16 = u16::from_be_bytes([
            value[TYPE_SECTION_SIZE_INDEX_0],
            value[TYPE_SECTION_SIZE_INDEX_1],
        ]);

        let number_of_code_sections: u16 = u16::from_be_bytes([
            value[NUMBER_OF_CODE_SECTIONS_INDEX_0],
            value[NUMBER_OF_CODE_SECTIONS_INDEX_1],
        ]);

        let code_section_size: u16 = u16::from_be_bytes([
            value[CODE_SECTION_SIZE_INDEX_0],
            value[CODE_SECTION_SIZE_INDEX_1],
        ]);

        let data_section_size: u16 = u16::from_be_bytes([
            value[DATA_SECTION_SIZE_INDEX_0],
            value[DATA_SECTION_SIZE_INDEX_1],
        ]);

        Ok(Header {
            version: value[VERSION_INDEX],
            type_section_size,
            number_of_code_sections,
            code_section_size,
            data_section_size,
        })
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn can_create_header_from_raw() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;

        let header = Header::try_from(raw).unwrap();

        assert_eq!(header.version(), Ok(Version::V1));
        assert_eq!(header.type_section_size(), 0);
        assert_eq!(header.number_of_code_sections(), 0);
        assert_eq!(header.code_section_size(), 0);
        assert_eq!(header.data_section_size(), 0);
    }

    #[test]
    fn can_get_codesize() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_1] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_1] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().codesize(), 3 + HEADER_SIZE);
    }

    #[test]
    fn can_get_two_byte_codesize() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_0] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_1] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_0] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_1] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_0] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().codesize(), 0x0303 + HEADER_SIZE);
    }

    #[test]
    fn can_get_version() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[VERSION_INDEX] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().version(), Ok(Version::V1));
    }

    #[test]
    fn cant_get_invalid_version() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[VERSION_INDEX] = 0x02;

        assert!(Header::try_from(raw).is_err());
    }

    #[test]
    fn can_get_type_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().type_section_size(), 1);
    }

    #[test]
    fn can_get_two_byte_type_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_0] = 0x01;
        raw[TYPE_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().type_section_size(), 0x0101);
    }

    #[test]
    fn can_get_number_of_code_sections() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[NUMBER_OF_CODE_SECTIONS_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().number_of_code_sections(), 1);
    }

    #[test]
    fn can_get_two_byte_number_of_code_sections() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[NUMBER_OF_CODE_SECTIONS_INDEX_0] = 0x01;
        raw[NUMBER_OF_CODE_SECTIONS_INDEX_1] = 0x01;

        assert_eq!(
            Header::try_from(raw).unwrap().number_of_code_sections(),
            0x0101
        );
    }

    #[test]
    fn can_get_code_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().code_section_size(), 1);
    }

    #[test]
    fn can_get_two_byte_code_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_0] = 0x01;
        raw[CODE_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().code_section_size(), 0x0101);
    }

    #[test]
    fn can_get_data_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().data_section_size(), 1);
    }

    #[test]
    fn can_get_two_byte_data_section_size() {
        let mut raw = DEFAULT_HEADER;
        raw[VERSION_INDEX] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_0] = 0x01;
        raw[DATA_SECTION_SIZE_INDEX_1] = 0x01;

        assert_eq!(Header::try_from(raw).unwrap().data_section_size(), 0x0101);
    }
}
