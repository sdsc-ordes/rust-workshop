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
    //TODO: 1. Implement here a `new` function to construct
    //         `UniqueIterator`.
}

// TODO: 3. Implement the `Iterator` trait for `UniqueIterator`.
//          Hint: You need to only provide one function, which one is it?
//          Hint: Also you need to define the associated type `Item`.
//
// TODO: 5. Implement the `UniqueIteratorExt` trait which provides a
//          function `fn unique(self) -> ?`.
//          What should it return?

fn main() {
    let values = [1, 2, 3, 4, 4, 2, 5];

    println!("All values:");
    values.iter().for_each(|x| println!("Value: {x}"));

    println!("Only unique values:");
    // TODO: 1. Implement a `new` function to construct a `UniqueIterator` from
    //          `values.iter()` and
    //          construct a `it_unique` variable here.

    // TODO: 3. Implement the iterator interface above,
    //          and delete this comment.

    // TODO: 4. Exhaust the iterator here
    //          by using it and printing values to stdout.

    // TODO: 5. Implement the `UniqueIteratorExt` trait above.

    println!("Only unique values, but now with `unique()`:");
    // TODO: 6. Use the `unique()` function on `values` and print
    //          the same as in 2.

    // Wow, you learned some awesome techniques and mechanism about
    // traits and iterators which make Rust great.
}
