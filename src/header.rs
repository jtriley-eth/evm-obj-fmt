use crate::{error::Error, version::Version};

pub mod constants {
    pub const MIN_HEADER_SIZE: usize = 15;
    pub const MAGIC_BYTE_0: u8 = 0xef;
    pub const MAGIC_BYTE_1: u8 = 0x00;
    pub const TYPE_SECTION_MARKER: u8 = 0x01;
    pub const CODE_SECTION_MARKER: u8 = 0x02;
    pub const DATA_SECTION_MARKER: u8 = 0x03;
    pub const TERMINATOR: u8 = 0x00;
}

pub struct Header {
    version: Version,
    type_section_size: u16,
    number_of_code_sections: u16,
    code_section_sizes: Vec<u16>,
    data_section_size: u16,
}

impl Header {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            type_section_size: 0,
            number_of_code_sections: 0,
            code_section_sizes: Vec::new(),
            data_section_size: 0,
        }
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn type_section_size(&self) -> u16 {
        self.type_section_size
    }

    pub fn number_of_code_sections(&self) -> u16 {
        self.number_of_code_sections
    }

    pub fn code_section_sizes(&self) -> &Vec<u16> {
        &self.code_section_sizes
    }

    pub fn data_section_size(&self) -> u16 {
        self.data_section_size
    }

    pub fn size(&self) -> usize {
        // 13 bytes + number of code sections * 2 bytes
        constants::MIN_HEADER_SIZE - 2 + self.number_of_code_sections * 2
    }
}
