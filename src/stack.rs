use std::fmt::{Display, Debug};
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialEq, PartialOrd, Eq};
use std::hash::Hash;

pub trait IntStack: Sized + 
    Display + Debug + PartialEq + PartialOrd + Eq + Hash + 
    Add<Output = Self> + Sub<Output = Self>  + Mul<Output = Self> + Div<Output = Self> {}
impl IntStack for u8 {}
impl IntStack for u16 {}
impl IntStack for u32 {}
impl IntStack for u64 {}
impl IntStack for usize {}
impl IntStack for i8 {}
impl IntStack for i16 {}
impl IntStack for i32 {}
impl IntStack for i64 {}
impl IntStack for isize {}

#[derive(Debug)]
pub struct Stack<T: IntStack> {
    m: Vec<T>
}

impl<T> Stack<T> 
    where T: IntStack {
    pub fn new() -> Stack<T> {
        Stack {
            m: Vec::new(),
        }
    }

    pub fn push(&mut self, v: T) {
        self.m.push(v);
    }

    pub fn pop(&mut self) -> T {
        self.m.pop().unwrap()
    }

    pub fn last(&self) -> T {
        self.m[self.m.len() - 1]
    }
}