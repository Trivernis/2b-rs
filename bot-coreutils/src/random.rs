use rand::seq::SliceRandom;

/// Chooses a random value from the given iterator
/// panics when the iterator is empty
pub fn choose_unchecked<'a, I: SliceRandom<Item = &'a T>, T>(i: I) -> &'a T {
    let mut rng = rand::thread_rng();

    i.choose(&mut rng).unwrap()
}
