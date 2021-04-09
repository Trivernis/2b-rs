use std::collections::VecDeque;

use rand::Rng;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;

/// Fisher-Yates shuffle for VecDeque
pub fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}
