# Generics

This folder contains some exercises to introduce you to generics.

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

## Information

Generics is the topic of generalizing types and functionalities to broader
cases. This is extremely useful for reducing code duplication in many ways, but
can call for some rather involved syntax. Namely, being generic requires taking
great care to specify over which types a generic type is actually considered
valid. The simplest and most common use of generics is for type parameters.

## Further Information

- [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
