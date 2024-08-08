# Traits

This folder contains some exercises to introduce you to traits.

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

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are
defined for the data type. For example, the `String` data type implements the
`From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract
classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when
writing generics.

## Further Information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
