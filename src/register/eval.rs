enum Instruction {
    HLT,
    NOP,
    ADD(i64, i64),
}

const MASK:i64 = 0xFF;

struct Evaluator {
    mem: Vec<i64>,

    // Registers
    ip: i64, // Instruction pointer
    sp: i64, // Stack pointer
    i: [i64; 8], // Int registers
    f: [f64; 8], // Float registers

    // Flags
    z: bool, // Zero
    l: bool, // Lower than zero
    g: bool, // Greater than zero

    // Current instruction
    inst: i64,
    dest: i64,
    src: i64,    
}

impl Evaluator {
    pub fn new(mem: Vec<i64>) -> Evaluator {
        Evaluator{
            mem,
            ip: -1,
            sp: mem_size - 1,
            i: [0; 8],
            f: [0.; 8],
            z: false,
            l: false,
            g: false,
            inst: 0,
            dest: 0,
            src: 0,
        }
    }

    fn reset_flags(&mut self) {
        self.z = false;
        self.l = false;
        self.g = false;
    }

    fn set_flags(&mut self, a:i64, b: i64) {
        let res = a - b;
        self.z = res == 0;
        self.l = res < 0;
        self.g = res > 0;
    }

    fn next(&mut self) -> Result<Instruction, &'static str> {
        self.ip += 1;
        match self.mem[self.ip as usize] & MASK {
            0x000000000000000 => Ok(Instruction::HLT),
            0x000000000000001 => Ok(Instruction::NOP),
            0x000000000000002 => {
                let ip = self.ip as usize;
                self.ip += 2;
                Ok(Instruction::ADD(self.mem[ip + 1], self.mem[ip + 2]))
            },
            _ => Err("Invalid instruction"),
        }
        // self.dest = self.ip+1;
        // self.src = self.ip+2;
    }

    fn exec(&self, i: &Instruction) -> Result<(), &'static str>{
        match i {
            Instruction::NOP => println!("NOP"),
            Instruction::ADD(a, b) => println!("ADD {}, {}", a, b),
            Instruction::HLT => panic!("HLT should not get here"),
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str>{
        loop {
            let instr = self.next()?;
            match instr {
                Instruction::HLT => return Ok(()),
                _ => match self.exec(&instr) {
                    Ok(_) => {},
                    Err(e) => return Err(e),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use eval::Evaluator;
    
    #[test]
    fn it_works() {
        let mut e = Evaluator::new(vec![0x0000000000000001, 0x0000000000000000], 2);
        assert_eq!(e.run(), Ok(()));
    }
}