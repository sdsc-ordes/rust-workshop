# Blanket Implementations

Blanket implementations come handy when you define a type `Banana` and you
implement a trait `Trait` on that type.

If you do `impl Trait for Banana` the trait `Trait` is NOT automatically
implemented on

- any reference to `&Banana` or `&mut Banana`,
- nor is it implemented for `Box<Banana>` or `Arc<Banana>`

This exercise helps you understand what blanket implementations for Traits are.
You should implement these when you define your Traits user will use to make
them not surprised that it does not work for references or Boxed values.

Follow the instructions in the comments of `src/bin/01.rs`!

This chapter is taken from advice in chapter 3, p. 40 in `Rust for Rustaceans`
from _J. Gjenset_.

To get started, run:

```bash
cargo run --bin 01
```

This will try to compile exercise 1.
