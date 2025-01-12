# Options

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

# Information

Type Option represents an optional value: every Option is either Some and
contains a value, or None, and does not. Option types are very common in Rust
code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range
  (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on
  error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
