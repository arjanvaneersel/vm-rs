enum Instruction {
    HLT,
    NOP,
}

const mask:i64 = 0xFF;

struct Evaluator {
    mem: i64,
    memSize: i64,

    // Registers
    ip: i64, // Instruction pointer
    sp: i64, // Stack pointer
    i: [i64; 8], // Int registers
    f: [f64; 8], // Float registers

    // Flags
    z: bool, // Zero
    l: bool, // Lower than zero
    g: bool, // Greater than zero

    // Current insturuction
    inst: i64,
    dest: i64,
    src: i64,    
}

impl Evaluator {
    pub fn new(mem: i64, memSize: i64) -> Evaluator {
        Evaluator{
            mem,
            memSize,
            ip: -1,
            sp: memSize - 1,
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

    fn ip_as_index(&self) -> usize {
        self.ip as usize
    }

    fn fetch(&mut self) {
        self.ip += 1;
        let r = self.mem[self.ip_as_index()] & mask;
        self.inst = r;
        self.dest = self.ip+1;
        self.src = self.ip+2;
    }

    fn exec(&self) {
        match &self.inst {
            0x01 => println!("NOP"),
            _ => {} 
        }
    }

    pub fn run(&mut self) {
        loop {
            self.fetch();
            match self.inst {
                0x00 => return,
                _ => self.exec(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}