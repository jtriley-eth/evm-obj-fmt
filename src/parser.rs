use crate::{
    container::type_section::FunctionMetadata, error::Error, header::constants::*,
    instruction::opcode::*, version::Version,
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

    let type_section_size = usize::from_be_bytes(bytecode[4..6].try_into().unwrap());

    if type_section_size % 4 != 0 {
        return Err(Error::TypeSectionMalformed);
    }

    let number_of_code_sections = usize::from_be_bytes(bytecode[6..8].try_into().unwrap());

    if number_of_code_sections != type_section_size / 4 {
        return Err(Error::NumberOfCodeSections);
    }

    // TODO: parse code sections as we go instead of allocating?
    let mut code_section_sizes = Vec::with_capacity(number_of_code_sections);
    let mut index: usize = 9;

    let end_code_seciton_sizes = number_of_code_sections * 2 + 9;

    while index < end_code_seciton_sizes {
        code_section_sizes.push(usize::from_be_bytes(
            bytecode[index..index + 2].try_into().unwrap(),
        ));

        index += 2;
    }

    if bytecode[index] != DATA_SECTION_MARKER || bytecode[index + 3] != TERMINATOR {
        return Err(Error::Header);
    }

    let data_section_size =
        usize::from_be_bytes(bytecode[index + 1..index + 3].try_into().unwrap());

    // index now points to the end of the header
    index += 4;

    let expected_len = index + code_section_sizes.iter().sum::<usize>() + data_section_size;

    if bytecode.len() < expected_len as usize {
        return Err(Error::Header);
    }

    // --- Parse Type Section ---
    let type_section_start = index;
    let type_section_end = index + type_section_size;

    while index < type_section_end {
        // let function_metadata = FunctionMetadata::from(&bytecode[index..index + 4]);

        // if index == type_section_start
        //     && (function_metadata.inputs() != 0 || function_metadata.outputs() != 0)
        // {
        //     return Err(Error::FirstCodeSection);
        // }

        index += 4;
    }

    // --- Parse Code Sections ---
    for (sectoion_index, code_section_size) in code_section_sizes.iter().enumerate() {
        let code_section = &bytecode[index..index + code_section_size];

        let mut stack_height = 0;
        let mut max_stack_height = 0;

        for opcode in code_section {
            let opcode = OpCode::try_from_u8(*opcode).ok_or(Error::InvalidOpcode)?;

            stack_height += opcode.pushes();
            if stack_height > max_stack_height {
                max_stack_height = stack_height;
                // if max_stack_height >
            }
        }
    }

    todo!()
}

/// Parse EOF Structured Bytecode.
///
/// Returns the code section start (assignable to program counter) or an EOF Error
pub unsafe fn _parse(bytecode: &[u8]) -> Result<usize, Error> {
    if bytecode.len() < MIN_HEADER_SIZE {
        return Err(Error::NotEOF);
    }

    let range = bytecode.as_ptr_range();
    let start = range.start;
    let end = range.end;
    let mut iterator = start;

    // --- PARSE HEADER ---
    let mut expected_codesize = HEADER_SIZE_NO_CODE_SECTIONS;

    if u16::from_be_bytes([*iterator, *iterator.offset(1)]) != MAGIC {
        return Err(Error::NotEOF);
    }

    let _ = Version::try_from(*iterator.offset(2))?;

    if *iterator.offset(3) != TYPE_SECTION_MARKER {
        return Err(Error::Header);
    }

    let type_section_size = u16::from_be_bytes([*iterator.offset(4), *iterator.offset(5)]);

    if *iterator.offset(6) != CODE_SECTION_MARKER {
        return Err(Error::Header);
    }

    let number_of_code_sections = u16::from_be_bytes([*iterator.offset(7), *iterator.offset(8)]);

    if type_section_size != number_of_code_sections * 4 {
        return Err(Error::NumberOfCodeSections);
    }

    iterator = iterator.offset(9);
    let code_section_sizes_start = iterator;
    expected_codesize += number_of_code_sections as usize * 2;

    if bytecode.len() < expected_codesize {
        return Err(Error::Header);
    }

    let mut index = 0;
    while index < number_of_code_sections {
        expected_codesize += u16::from_be_bytes([*iterator, *iterator.offset(1)]) as usize;
        iterator = iterator.offset(2);
        index += 1;
    }

    if *iterator != DATA_SECTION_MARKER {
        return Err(Error::Header);
    }

    let data_section_size = u16::from_be_bytes([*iterator.offset(1), *iterator.offset(2)]);

    if *iterator.offset(3) != TERMINATOR {
        return Err(Error::Header);
    }

    iterator = iterator.offset(4);

    // --- PARSE TYPE SECTION ---
    let mut index = 0;

    while index < type_section_size {
        let metadata = FunctionMetadata::from([
            *iterator,
            *iterator.offset(1),
            *iterator.offset(2),
            *iterator.offset(3),
        ]);

        if index == 0 && metadata.inputs() != 0 || metadata.outputs() != 0 {
            return Err(Error::FirstCodeSection);
        }

        iterator = iterator.offset(4);
        index += 4;
    }

    todo!()
}
