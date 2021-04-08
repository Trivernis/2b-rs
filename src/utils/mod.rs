use std::collections::VecDeque;

use rand::Rng;

pub mod error;
pub mod store;

/// Fisher-Yates shuffle for VecDeque
pub fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}
