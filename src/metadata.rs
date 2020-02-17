#[derive(Clone)]
pub struct Metadata {
    pub name: String,
    pub nargs: isize,
    pub nlocals: isize,
    pub address: usize, // bytecode address
}

impl Metadata {
    pub fn new(name: String, nargs: isize, nlocals: isize, address: usize) -> Metadata {
        Metadata {
            name,
            nargs,
            nlocals,
            address,
        }
    } 
}