use crate::{
    error::Error,
    header::parse_header,
};

pub fn parse(bytecode: &[u8]) -> Result<Container, Error> {
    let header = parse_header(bytecode);

    // if `Header::version` returns an error, bubble it up as an error
    let _ = header.version()?;

    if header.codesize() != bytecode.len() as u64 {
        return Err(Error::BytecodeLength);
    }
}
