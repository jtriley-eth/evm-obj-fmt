use crate::{
    container::type_section::FunctionMetadata,
    error::Error,
    header::{DEFAULT_MASK, HEADER_SIZE, parse_header},
};

pub fn parse(bytecode: &[u8]) -> Result<Container, Error> {
    let header = parse_header(bytecode)?;

    // if `Header::version` returns an error, bubble it up as an error
    let _ = header.version()?;

    if header.codesize() != bytecode.len() as u64 {
        return Err(Error::BytecodeLength);
    }

    let type_section = parse_types(header, bytecode)?;

    // TODO: parse code and data section, validate
}

fn parse_header(raw: &[u8]) -> Result<Header, Error> {
    let header = Header::from(raw);

    return if header.0 & 0xff_u128 != 0 || header.0 & DEFAULT_MASK != DEFAULT_MASK {
        return Err(Error::Header);
    } else {
        Ok(header)
    }
}

fn parse_types(header: Header, raw: &[u8]) -> Result<TypeSection, Error> {
    let type_section_slice = raw[HEADER_SIZE..HEADER_SIZE + header.type_section_size() as usize];

    if type_section_slice.len() % 4 != 0 {
        return Err(Error::TypeSection);
    }

    let types_len = type_section_slice.len() / 4;

    let types: Vec<FunctionMetadata> = Vec::with_capacity(types_len);

    for index in 0..types_len {
        let start = index * 4;
        let end = start + 4;

        types.push(FunctionMetadata::from(&type_section_slice[start..end]));
    }

    Ok(TypeSection(types))
}
