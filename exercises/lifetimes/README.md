# Lifetimes

This folder contains some exercises to introduce you to lifetimes.

To get started, run:

```bash
cargo run --bin 01
```

This will try to compile exercise 1. Try and get the example to run, and
continue on with the next exercise by replacing the number of the exercise in
the cargo run command.

Some exercises contain unit tests. To run the test in `src/bin/01.rs`, run

```bash
cargo test --bin 01
```

Make sure all tests pass!

# Information

Lifetimes tell the compiler how to check whether references live long enough to
be valid in any given situation. For example lifetimes say "make sure parameter
'a' lives as long as parameter 'b' so that the return value is valid".

They are only necessary on borrows, i.e. references, since copied parameters or
moves are owned in their scope and cannot be referenced outside. Lifetimes mean
that calling code of e.g. functions can be checked to make sure their arguments
are valid. Lifetimes are restrictive of their callers.

If you'd like to learn more about lifetime annotations, the
[lifetimekata](https://tfpk.github.io/lifetimekata/) project has a similar style
of exercises to Rustlings, but is all about learning to write lifetime
annotations.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
