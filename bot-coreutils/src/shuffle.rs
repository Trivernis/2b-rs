use rand::Rng;
use std::collections::VecDeque;

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for VecDeque<T> {
    /// Fisher-Yates shuffle implementation
    /// for VecDeque.
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut i = self.len();

        while i >= 2 {
            i -= 1;
            self.swap(i, rng.gen_range(0..i + 1))
        }
    }
}
