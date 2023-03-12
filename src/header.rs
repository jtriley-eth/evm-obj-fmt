use crate::{error::Error, version::Version};

const HEADER_SIZE: usize = 15;

const DEFAULT_MASK: u128 = 0x00_ef_00_00_01_00_00_02_00_00_00_00_03_00_00_00;

const VERSION_MASK: u128 = 0x00_00_00_ff_00_00_00_00_00_00_00_00_00_00_00_00;
const TYPE_SECTION_SIZE_MASK: u128 = 0x00_00_00_00_00_ff_ff_00_00_00_00_00_00_00_00_00;
const NUMBER_OF_CODE_SECTIONS_MASK: u128 = 0x00_00_00_00_00_00_00_00_ff_ff_00_00_00_00_00_00;
const CODE_SECTION_SIZE_MASK: u128 = 0x00_00_00_00_00_00_00_00_00_00_ff_ff_00_00_00_00;
const DATA_SECTION_SIZE_MASK: u128 = 0x00_00_00_00_00_00_00_00_00_00_00_00_00_ff_ff_00;

pub struct Header(u128);

impl Header {
    /// Returns Total Code Size
    pub fn codesize(&self) -> u64 {
        HEADER_SIZE as u64
            + self.type_section_size() as u64
            + self.code_section_size() as u64
            + self.data_section_size() as u64
    }

    /// Returns the version
    pub fn version(&self) -> Result<Version, Error> {
        Version::try_from((self.0 & VERSION_MASK >> 96) as u8)
    }

    /// Returns the type section size
    pub fn type_section_size(&self) -> u16 {
        (self.0 & TYPE_SECTION_SIZE_MASK >> 72) as u16
    }

    /// Returns the number of code sections
    pub fn number_of_code_sections(&self) -> u16 {
        (self.0 & NUMBER_OF_CODE_SECTIONS_MASK >> 48) as u16
    }

    /// Returns the code section size
    pub fn code_section_size(&self) -> u16 {
        (self.0 & CODE_SECTION_SIZE_MASK >> 32) as u16
    }

    /// Returns the data section size
    pub fn data_section_size(&self) -> u16 {
        (self.0 & DATA_SECTION_SIZE_MASK >> 8) as u16
    }
}

impl From<&[u8]> for Header {
    fn from(value: &[u8]) -> Self {
        if value.len() < HEADER_SIZE {
            // TODO: remove panic
            panic!("bytecode len")
        }

        Header(u128::from_be_bytes([
            0x00, value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14],
        ]))
    }
}

pub fn parse_header(raw: &[u8]) -> Result<Header, Error> {
    let header = Header::from(raw);

    return if header.0 & 0xff_u128 != 0 || header.0 & DEFAULT_MASK != DEFAULT_MASK {
        return Err(Error::Header);
    } else {
        Ok(header)
    }
}
