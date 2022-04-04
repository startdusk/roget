use crate::{Correctness, Guesser};

#[derive(Debug, Clone, Copy)]
pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Guesser for Naive {
    fn guess(&mut self, hostory: &[crate::Guess]) -> String {
        todo!()
    }
}
