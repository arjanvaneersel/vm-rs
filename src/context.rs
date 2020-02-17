use std::collections::HashMap;
use crate::stack::IntStack;

#[derive(Clone)]
pub struct Context<T: IntStack> {
    pub return_ip: T,
    pub locals: HashMap<T, T>,
}

impl<T> Context<T> where T: IntStack {
    pub fn new(return_ip: T) -> Context<T> {
        Context {
            return_ip,
            locals: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::context::*;

    #[test]
    fn it_works() {
        
    }
}