use core::num::{dec2flt::number, self};

use crate::{
    container::type_section::FunctionMetadata,
    error::Error,
    header::{constants::*, parse_header},
    version::Version,
};

/*
- check magic
- check version
- check type section marker
- check type section size
- check code section marker
- check number of code sections

- check type section size vs number of code sections
- iterate over type section + code section
    - check inputs
    - check outputs
    - check max stack height

- check code section sizes
- check data section marker
- check data section size
*/

pub fn parse(bytecode: &[u8]) -> Result<(), Error> {
    // --- Parse Header ---

    // TODO: break out header parsing
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

    if type_section_size % 4 != 0 {
        return Err(Error::TypeSectionMalformed);
    }

    let number_of_code_sections = u16::from_be_bytes(bytecode[6..8]);

    if number_of_code_sections != type_section_size / 4 {
        return Err(Error::NumberOfCodeSections);
    }

    // TODO: parse code sections as we go instead of allocating?
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

    // --- Parse Type Section ---
    let type_section_end = index + type_section_size;

    while index < type_section_end {
        let function_metadata = FunctionMetadata::try_from(bytecode[index..index+4])?;

        if current_type_section == 0
            && (function_metadata.inputs != 0 || function_metadata.outputs != 0) {
            return Err(Error::FirstCodeSection);
        }

        index += 4;
    }

    // --- Parse Code Sections ---
    for code_section_size in code_section_sizes.iter() {
        let code_section = bytecode[index..index+code_section_size];
        let mut stack_depth = 0;

        

        for opcode in code_section {
            let opcode = OpCode::try_from(*opcode).ok_or(Error::InvalidOpcode)?;


        }
    }

    todo!()
}
