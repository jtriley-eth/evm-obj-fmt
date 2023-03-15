use crate::{
    container::type_section::FunctionMetadata,
    error::Error,
    header::{constants::*, parse_header},
    version::Version,
};

pub fn parse(bytecode: &[u8]) -> Result<(), Error> {
    if bytecode.len() < MIN_HEADER_SIZE
        || bytecode[0] != MAGIC_BYTE_0
        || bytecode[1] != MAGIC_BYTE_1
    {
        return Err(Error::NotEOF);
    }

    let version = Version::try_from(bytecode[2])?;

    if bytecode[3] != TYPE_SECTION_MARKER || bytecode[6] != CODE_SECTION_MARKER {
        return Err(Error::Header);
    }

    let type_section_size = u16::from_be_bytes(bytecode[4..6]);
    let number_of_code_sections = u16::from_be_bytes(bytecode[6..8]);

    let mut code_section_sizes = Vec::with_capacity(number_of_code_sections);
    let mut index = 9;

    let end_code_seciton_sizes = number_of_code_sections * 2 + 9;

    while index < end_code_seciton_sizes {
        code_section_sizes.push(u16::from_be_bytes(bytecode[index..index + 2]));

        index += 2;
    }

    if bytecode[index] != DATA_SECTION_MARKER || bytecode[index + 3] != TERMINATOR {
        return Err(Error::Header);
    }

    let data_section_size = u16::from_be_bytes(bytecode[index + 1..index + 3]);

    // index now points to the end of the header
    index += 4;

    let expected_len = index + code_section_sizes.iter().sum::<u16>() + data_section_size as usize;

    if bytecode.len() < expected_len {
        return Err(Error::Header);
    }

    for code_section_size in code_section_sizes.iter() {
        let code_section = &bytecode[index..index+code_section_size];

        for opcode in code_section {
            let op = opcode.into();
        }
    }

    todo!()
}
