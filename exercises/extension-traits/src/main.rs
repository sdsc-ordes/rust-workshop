use std::{collections::HashSet, hash::Hash};
// This exercise introduces extension traits. A
// mechanism to extend the functionality on types.
// We will extend in this exercise the functionality on
// iterators.
//
// The orphan rule says that you can only:
//
// - implement YOUR trait for an external type (in another library)
// - implement an external trait for YOUR type.

// This trait acts on types which also implement the Iterator trait
// It transforms the type to an UniqueIterator by `unique()`
// which produces only unique values.
// trait UniqueIteratorExt: Iterator {
//     fn unique(self) -> UniqueIterator<Self>
//     where
//         Self: Sized,
//         Self::Item: Eq + Hash + Clone,
//     {
//         return UniqueIterator {
//             iter: self,
//             visited: HashSet::new(),
//         };
//     }
// }

// This is the iterator type which
// produces only unique values.
struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    iter: I,                   // Stores the original iterator.
    visited: HashSet<I::Item>, // Stores all already visited types here.
}

impl<I> UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    fn new(it: I) -> UniqueIterator<I> {
        UniqueIterator {
            iter: it,
            visited: HashSet::new(),
        }
    }
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.find(|x| self.visited.insert(x.clone()));
    }
}

fn main() {
    let values = [1, 2, 3, 4, 4, 2, 5];

    println!("All values:");
    values.iter().for_each(|x| println!("Value: {x}"));

    println!("Only unique values:");

    let it = values.iter();
    let unique_it = UniqueIterator::new(it);
    unique_it.for_each(|x| println!("Value: {x}"))
}
