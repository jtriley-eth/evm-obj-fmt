pub mod opcode;
use opcode::{OpCode, OpCodeMetadata};

use crate::error::Error;

// TODO: refactor all of this based on revm_interpreter changes

pub struct Instruction<const IMMEDIATES: usize> {
    opcode: OpCode,
    immediate: [u8; IMMEDIATES]
}

// TODO: better handling of the `rjumpv <len> <jumptable>` instruction.
impl<const IMMEDIATES: usize> Instruction<IMMEDIATES> {
    /// Try to create a new instruction
    /// 
    /// Fails if `IMMEDIATES` does not match the opcode minimum immediates
    pub fn try_new(opcode: OpCode, immediate: [u8; IMMEDIATES]) -> Result<Self, Error> {
        if  IMMEDIATES as u8 != opcode.minimum_immediates() {
            return Err(Error::OpCodeImmediates)
        }

        Ok(Self {
            opcode,
            immediate
        })
    }
}
