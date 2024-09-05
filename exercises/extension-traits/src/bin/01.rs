// This exercise introduces extension traits. A
// mechanism to extend the functionality on types.
//
// We will extend in this exercise the functionality on
// ALL iterators to have a `unique()` function which returns a new iterator
// `UniqueIterator` which we implement.
//
// The orphan rule says that you can only do either
//
// 1. implement an external trait for YOUR type.
// or
// 2. implement YOUR trait for an external type (in another library)
//
// In this exercise we are using both in succession:
//
// -  We use 1. to implement the external `Iterator`
//    for our type `UniqueIterator`.
//
// -  Then we are using 2.: We create our own trait `UniqueIteratorExt`
//    which provides a function `unique()`. This trait we make
//    auto-implemented for all iterators (blanked implementation).
//    In that way we can use `it.unique()` for each iterator `it`.
//
// NOTE: Follow the todo numbers in sequence!
//
use std::{collections::HashSet, hash::Hash};

// UniqueIterator is the iterator type which produces only unique values.
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
    // Construct a new unique iterator from an existing one.
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

trait UniqueIteratorExt: Iterator {
    // Note: `Self` here is the iterator type for
    // which this UniqueIteratorExt gets implemented.
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        UniqueIterator::<Self>::new(self) // you can also omit the `::<Self>`
    }
}

impl<T: Iterator> UniqueIteratorExt for T {}

fn main() {
    let values = [1, 2, 3, 4, 4, 2, 5];

    println!("All values:");
    values.iter().for_each(|x| println!("Value: {x}"));

    println!("Only unique values:");
    let it = values.iter();
    let unique_it = UniqueIterator::new(it);
    unique_it.for_each(|x| println!("Value: {x}"));

    println!("Only unique values, but now with `unique()`:");
    values.iter().unique().for_each(|x| println!("Value: {x}"));
}
