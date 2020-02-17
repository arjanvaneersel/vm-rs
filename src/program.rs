use std::convert::TryFrom;

use crate::opcode::Instruction;

pub struct Program {
    program: Vec<u8>
}

impl Program {
    pub fn new(program: Vec<u8>) -> Program {
        Program { program }
    }

    pub fn fetch(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.program.len() {
            Ok(self.program[index])
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn fetch_instruction(&self, index: usize) -> Result<Instruction, &'static str> {
        Ok(Instruction::try_from(self.fetch(index)?)?)
    }

    // pub fn fetch_word(&self, index: usize) -> Result<u64, &'static str> {
    //     let i = index + WORD_SIZE;

    //     if i <= self.program.len() {
    //         let w = &self.program[index..i];
    //         bytes_to_word(w)
    //     } else {
    //         Err("Index out of bounds")
    //     }
    // }
}