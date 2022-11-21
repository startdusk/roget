use crate::Guesser;

pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new()
    }
}

impl Guesser for Naive {
    fn guess(&mut self, _history: &[crate::Guess]) -> String {
        todo!()
    }
}
