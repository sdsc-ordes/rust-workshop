# Drop with Error Handling

This exercise practices to implement `Drop` on a type while retaining error
handling.

We have a type `TempFile` which should simulate some resource which you acquire
and initialize with `new`
([RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization))
and which implements `Drop` which will remove the file from the disc whenever it
gets dropped.

Returning errors from `fn Drop::drop(&mut self)` is not possible. We could
ignore errors, but in this exercise we want to **leave no loose** ends and not
just ignore errors. That is why we implement an explicit destructor `close`
which takes ownership by taking `self` as value and returns
`Result<(), ErrorHandle>` in case something happens.

The code below compiles, but if you try to call `remove` in `close` the compiler
will tell you that you cannot partially move out values before `Drop::drop` is
called at function end. Also calling `remove` in `Drop` does not work because
`Drop::drop` does not own `self` it only has a `&mut self`.

The exercise is to solve these issues by wrapping `TempFile` into an `Option`
and implementing `Drop::drop` on the wrapped type leveraging the function
`Option<T>::take`.

This exercise was inspired from advice in ch. 3, p. 46 in `Rust for Rustaceans`
from _J. Gjenset_.

To get started, run:

```bash
cargo run --bin 01
```

This will try to compile exercise 1.
