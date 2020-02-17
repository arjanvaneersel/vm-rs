use std::convert::TryFrom;
use std::fmt;

pub const NOP: u8 = 0;
pub const IADD: u8 = 1;       // Pop last to integers on the stack and push their sum
pub const ISUB: u8 = 2;
pub const IMUL: u8 = 3;
pub const ILT: u8 =  4;       // Less than
pub const IEQ: u8 = 5;        // Equal to
pub const BR: u8 = 6;         // Branch
pub const BRT: u8 = 7;        // Branch when true
pub const BRF: u8 = 8;        // Branch when false
pub const IPUSH: u8 = 9;      // Push int to stack
pub const LOAD: u8 = 10;      // Load from local register
pub const GLOAD: u8 = 11;     // Load from global register
pub const STORE: u8 = 12;     // Store in local register
pub const GSTORE: u8 = 13;    // Store in global register
pub const PRINT: u8  = 14;    // Print stack head
pub const POP: u8 = 15;       // Pop stack head
pub const CALL: u8 = 16;
pub const RETURN: u8 = 17;
pub const DONE: u8 = 18;      // Stop execution

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    NOP,
    IADD, 
    ISUB, 
    IMUL, 
    ILT, 
    IEQ,  
    BR,   
    BRT,  
    BRF,  
    IPUSH,
    LOAD,
    GLOAD,
    STORE,
    GSTORE,
    PRINT,
    POP,
    CALL,
    RETURN,   
    DONE,
}

impl Instruction {
    pub fn args(self) -> usize {
        match self {
            Instruction::NOP => 0,
            Instruction::IADD => 0, 
            Instruction::ISUB => 0, 
            Instruction::IMUL => 0, 
            Instruction::ILT => 0, 
            Instruction::IEQ => 0,  
            Instruction::BR => 1,   
            Instruction::BRT => 1,  
            Instruction::BRF => 1,  
            Instruction::IPUSH => 1,
            Instruction::LOAD => 1,
            Instruction::GLOAD => 1,
            Instruction::STORE => 1,
            Instruction::GSTORE => 1,
            Instruction::PRINT => 0,
            Instruction::POP => 0, 
            Instruction::CALL => 1,
            Instruction::RETURN => 0,
            Instruction::DONE => 0,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::IADD => write!(f, "IADD"), 
            Instruction::ISUB => write!(f, "ISUB"), 
            Instruction::IMUL => write!(f, "IMUL"),
            Instruction::ILT => write!(f, "ILT"),
            Instruction::IEQ => write!(f, "IEQ"),
            Instruction::BR => write!(f, "BR"),
            Instruction::BRT => write!(f, "BRT"),
            Instruction::BRF => write!(f, "BRF"),
            Instruction::IPUSH => write!(f, "IPUSH"),
            Instruction::LOAD => write!(f, "LOAD"),
            Instruction::GLOAD => write!(f, "GLOAD"),
            Instruction::STORE => write!(f, "STORE"),
            Instruction::GSTORE => write!(f, "GSTORE"),
            Instruction::PRINT => write!(f, "PRINT"),
            Instruction::POP => write!(f, "POP"),
            Instruction::CALL => write!(f, "CALL"),
            Instruction::RETURN => write!(f, "RETURN"),
            Instruction::DONE => write!(f, "DONE"),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(v: Instruction) -> u8 {
        match v {
            Instruction::NOP => NOP,
            Instruction::IADD => IADD,
            Instruction::ISUB => ISUB, 
            Instruction::IMUL => IMUL, 
            Instruction::ILT => ILT, 
            Instruction::IEQ => IEQ,  
            Instruction::BR => BR,   
            Instruction::BRT => BRT,  
            Instruction::BRF => BRF,  
            Instruction::IPUSH => IPUSH,
            Instruction::LOAD => LOAD,
            Instruction::GLOAD => GLOAD,
            Instruction::STORE => STORE,
            Instruction::GSTORE => GSTORE,
            Instruction::PRINT => PRINT,
            Instruction::POP => POP,
            Instruction::CALL => CALL,
            Instruction::RETURN => RETURN,   
            Instruction::DONE => DONE,
        }
    }
}

impl TryFrom<u8> for Instruction {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            NOP => Ok(Instruction::NOP),
            IADD => Ok(Instruction::IADD),
            ISUB => Ok(Instruction::ISUB), 
            IMUL => Ok(Instruction::IMUL), 
            ILT => Ok(Instruction::ILT), 
            IEQ => Ok(Instruction::IEQ),
            BR => Ok(Instruction::BR), 
            BRT => Ok(Instruction::BRT), 
            BRF => Ok(Instruction::BRF), 
            IPUSH => Ok(Instruction::IPUSH),
            LOAD => Ok(Instruction::LOAD),
            GLOAD => Ok(Instruction::GLOAD),
            STORE => Ok(Instruction::STORE),
            GSTORE => Ok(Instruction::GSTORE),
            PRINT => Ok(Instruction::PRINT),
            POP => Ok(Instruction::POP),
            CALL => Ok(Instruction::CALL),
            RETURN => Ok(Instruction::RETURN),
            DONE => Ok(Instruction::DONE),
            _ => Err("Unknown instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::opcode::*;
    use std::convert::TryFrom;

    #[test]
    fn it_works() {
        let o = Instruction::try_from(IADD).unwrap();
        assert_eq!(o, Instruction::IADD);
        assert_eq!(o.args(), 0);
    }
}
