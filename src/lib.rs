pub mod vm;
mod opcode;
mod program;
mod stack;
mod context;
// mod context;
// mod metadata;

// use crate::opcode::*;
// use crate::context::Context;
// use crate::metadata::Metadata;

// use std::string::String;
// use std::collections::HashMap;

// pub struct VM {
//     pub trace: bool,
//     code: Vec<i32>,
//     stack: Vec<i32>,
//     // }
//     // self.ip = self.metadata[fi].address;
//     globals: HashMap<i32, i32>,
//     ip: usize,
//     // sp: i32,
//     fp: usize,
//     // ctx: Context,
//     metadata: Vec<Metadata>,
// }

// impl VM {
//     pub fn new(code: Vec<i32>, main: usize) -> VM {
//         let md: Vec<Metadata> = vec![Metadata::new(String::from(""), 0 , 0, 0)];
//         VM{
//             trace: false,
//             code,
//             stack: Vec::new(),
//             globals: HashMap::new(),
//             ip: main,
//             // sp: -1,
//             fp: 0,
//             // ctx: Context::new(None, 0, md[0].clone()),
//             metadata: md,
//         }
//     }

//     pub fn start(&mut self) -> Result<(), String> {
//         while self.ip < self.code.len() {
//             // Fetch opcode
//             let opcode = opcode::Opcode::from(self.code[self.ip]);
            
//             // String for collecting output
//             let mut output = String::new();

//             // Print the current instruction and arguments if trace is on
//             if self.trace {
//                 print!("{}", self.disassemble());
//             }

//             // Increase instruction pointer
//             self.ip += 1;

//             // Decode opcode
//             match opcode {
//                 Opcode::INVALID => return Err(format!("invalid opcode at {}", self.ip)),
//                 Opcode::IADD => {
//                     let b = self.stack.pop().unwrap();
//                     let a = self.stack.pop().unwrap();
//                     self.stack.push(a + b);
//                 }, 
//                 Opcode::ISUB => {
//                     let b = self.stack.pop().unwrap();
//                     let a = self.stack.pop().unwrap();
//                     self.stack.push(a - b);
//                 }, 
//                 Opcode::IMUL => {
//                     let b = self.stack.pop().unwrap();
//                     let a = self.stack.pop().unwrap();
//                     self.stack.push(a * b);
//                 }, 
//                 Opcode::ILT => {
//                     let b = self.stack.pop().unwrap();
//                     let a = self.stack.pop().unwrap();
//                     self.stack.push((a < b) as i32);
//                 }, 
//                 Opcode::IEQ => {
//                     let b = self.stack.pop().unwrap();
//                     let a = self.stack.pop().unwrap();
//                     self.stack.push((a == b) as i32);
//                 },  
//                 Opcode::BR => {
//                     let ip = self.ip as usize;
//                     self.ip = self.code[ip] as usize;
//                     self.ip += 1;
//                 },   
//                 Opcode::BRT => {
//                     let addr = self.code[self.ip];
//                     self.ip += 1;
//                     if self.stack[self.stack.len() - 1] == 1 { self.ip = addr as usize}
//                 },  
//                 Opcode::BRF => {
//                     let ip = self.ip as usize;
//                     let addr = self.code[ip];
//                     self.ip += 1;
//                     if self.stack[self.stack.len() - 1] == 0 { self.ip = addr as usize}
//                 },  
//                 Opcode::IPUSH => {
//                     self.stack.push(self.code[self.ip]);
//                     self.ip += 1;
//                 },
//                 Opcode::LOAD => {
//                     let r = self.code[self.ip];
//                     self.ip += 1;
//                     self.stack.push(self.ctx.locals[&r])
//                 },
//                 Opcode::GLOAD => {
//                     let addr = self.code[self.ip];
//                     self.ip += 1;
//                     self.stack.push(self.globals[&addr])
//                 },
//                 Opcode::STORE => {
//                     let v = self.stack.pop().unwrap();
//                     let r = self.code[self.ip];
//                     self.ip += 1;
//                     self.ctx.locals.insert(r, v);
//                 },
//                 Opcode::GSTORE => {
//                     let v = self.stack.pop().unwrap();
//                     let addr = self.code[self.ip];
//                     self.ip += 1;
//                     self.globals.insert(addr, v);
//                 },
//                 Opcode::PRINT => output.push_str(&format!("{}", self.stack[self.stack.len()-1])),
//                 Opcode::POP => {
//                     self.stack.pop().unwrap();
//                 }, 
//                 Opcode::CALL => {
//                     // let fi = self.code[self.ip] as usize;
//                     // self.ip += 1;
//                     // let na = self.metadata[fi].nargs;
//                     // self.ctx = Context::new(Some(self.ctx), self.ip, self.metadata[fi].clone());
//                     // for i in 0..na {
//                     //     self.ctx.locals[&(i as i32)] = self.stack.pop().unwrap();
//                     // }
//                     // self.ip = self.metadata[fi].address;
//                 },
//                 Opcode::RETURN => {
//                     // self.ip = self.ctx.return_ip;
//                     // self.ctx = self.ctx.get_parent();
//                 },
//                 Opcode::DONE => break,
//             }

//             // Print the stack if trace is on
//             if self.trace {
//                 println!("\t{:?}\t{:?}", self.stack, self.globals);
//             }

//             // Print the output if any
//             if output.len() > 0 {
//                 println!("[OUTP]\t{}", output)
//             }
//         }

//         Ok(())
//     }

//     fn disassemble(&self) -> String {
//         let mut s = String::new();
//         let ip = self.ip.clone();
//         let opcode = opcode::Opcode::from(self.code[ip]);

//         s.push_str(&format!("{:04}\t{:?}", ip, opcode));
//         match opcode.args() {
//             1 => s.push_str(&format!("\t{}", self.code[ip + 1].clone())),
//             2 => s.push_str(&format!("\t{} {}", self.code[ip + 1].clone(), self.code[ip + 2].clone())),
//             _ => s.push_str("\t   "),
//         }
//         s
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::opcode::*;
//     use crate::{VM};

//     #[test]
//     fn it_works() {
//         let code = vec![IPUSH, 2, IPUSH, 3, IADD, GSTORE, 0, IPUSH, 30, PRINT, GLOAD, 0, PRINT, DONE];

//         let mut vm = VM::new(code, 0);
//         vm.trace = true;
//         let res = vm.start();
//         assert_eq!(res.is_ok(), true)
//     }
// }
