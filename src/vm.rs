use std::string::String;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::opcode::Instruction;
use crate::program::Program;
use crate::stack::{Stack, IntStack};
use crate::context::Context;

pub struct VM<T: IntStack> {
    pub trace: bool,
    program: Program,
    stack: Stack<T>,
    ip: usize, // Instruction pointer
    globals: HashMap<T, T>,
    ctx: Context<T>,
}

impl<T> VM<T> 
    where T: IntStack {
    pub fn new(program: Vec<u8>, main: usize) -> VM<T> {
        VM{
            trace: false,
            program: Program::new(program),
            stack: Stack::new(),
            ip: main,
            globals: HashMap::new(),
            ctx: Context::new(0),
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let instr = self.program.fetch_instruction(self.ip)?;
            // let instr = self.decode(opcode)?;
            if self.trace {
                print!("{}", self.disassemble());
            }

            if instr == Instruction::DONE {
                return Ok(());
            }
            let output = self.exec(instr);
            
            // Print the stack if trace is on
            if self.trace {
                println!("\t{:?}\t{:?}", self.stack, self.globals);
            }

            // Print output if any exists
            if output.len() > 0 {
                println!("[OUTP]\t{}", output)
            }
        }
    }

    fn disassemble(&self) -> String {
        let mut s = String::new();
        match self.program.fetch_instruction(self.ip) {
            Ok(i) => {
                s.push_str(&format!("{:04}\t{}", self.ip, i));
                match i.args() {
                    1 => s.push_str(&format!("\t{}", 
                            self.program.fetch(self.ip + 1).unwrap().clone())),
                    2 => s.push_str(&format!("\t{} {}", 
                            self.program.fetch(self.ip + 1).unwrap().clone(), 
                            self.program.fetch(self.ip + 2).unwrap().clone())),
                    _ => s.push_str("\t   "),
                }
            },
            Err(e) => s.push_str(&format!("Invalid instruction: {}", e)),
        };
        s
    }

    // fn decode(&self, opcode: u8) -> Result<Instruction, &'static str> {
    //     Instruction::try_from(opcode)
    // }

    fn exec(&mut self, i: Instruction) -> String {
        let mut output = String::from("");

        self.ip += 1;
        match i {
            Instruction::NOP => {},
            Instruction::IADD => {
                let b = self.stack.pop();
                let a = self.stack.pop();
                self.stack.push(a + b);
            }, 
            Instruction::ISUB => {
                let b = self.stack.pop();
                let a = self.stack.pop();
                self.stack.push(a - b);
            }, 
            Instruction::IMUL => {
                let b = self.stack.pop();
                let a = self.stack.pop();
                self.stack.push(a * b);
            }, 
            Instruction::ILT => {
                let b = self.stack.pop();
                let a = self.stack.pop();
                self.stack.push(a < b);
            }, 
            Instruction::IEQ => {
                let b = self.stack.pop();
                let a = self.stack.pop();
                self.stack.push(a == b);
            },  
            Instruction::BR => {
                self.ip = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
            },   
            Instruction::BRT => {
                let addr = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                if self.stack.last() == 1 { self.ip = addr }
            },  
            Instruction::BRF => {
                let addr = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                if self.stack.last() == 0 { self.ip = addr }
            },  
            Instruction::IPUSH => {
                self.stack.push(self.program.fetch(self.ip).unwrap());
                self.ip += 1;
            },
            Instruction::LOAD => {
                let r = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                self.stack.push(self.ctx.locals[&r])
            },
            Instruction::GLOAD => {
                let addr = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                self.stack.push(self.globals[&addr])
            },
            Instruction::STORE => {
                let v = self.stack.pop();
                let r = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                self.ctx.locals.insert(r, v);
            },
            Instruction::GSTORE => {
                let v = self.stack.pop();
                let addr = self.program.fetch(self.ip).unwrap();
                self.ip += 1;
                self.globals.insert(addr, v);
            },
            Instruction::PRINT => output.push_str(&format!("{}", self.stack.last())),
            Instruction::POP => {
                self.stack.pop();
            }, 
            Instruction::CALL => {
                // let fi = self.code[self.ip];
                // self.ip += 1;
                // let na = self.metadata[fi].nargs;
                // self.ctx = Context::new(Some(self.ctx), self.ip, self.metadata[fi].clone());
                // for i in 0..na {
                //     self.ctx.locals[&(i as i32)] = self.stack.pop().unwrap();
                // }
                // self.ip = self.metadata[fi].address;
            },
            Instruction::RETURN => {
                // self.ip = self.ctx.return_ip;
                // self.ctx = self.ctx.get_parent();
            },
            Instruction::DONE => panic!("DONE should not reach exec"),
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::opcode::*;
    use crate::vm::VM;

    #[test]
    fn it_works() {
        let code = vec![IPUSH, 2, IPUSH, 3, IADD, GSTORE, 0, IPUSH, 30, PRINT, GLOAD, 0, PRINT, DONE];

        let mut vm = VM::<i64>::new(code, 0);
        vm.trace = true;
        let res = vm.run();
        assert_eq!(res.is_ok(), true)
    }
}