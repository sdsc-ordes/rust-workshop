# Proc Macros

This exercise is an exercise following the
[video](https://www.youtube.com/watch?v=SMCRQj9Hbx8) about procedural macros.

The procedural macro `comp![...syntax...]` in this exercise is by the end of the
day a Rust library function which parses the custom defined syntax (`...`) and
turns it into Rust code again which is then compiled by `rustc`.

In this exercise you are following the
[video](https://www.youtube.com/watch?v=SMCRQj9Hbx8) while implementing
directly. The setup is done for you, such that you don't need to worry about
libraries and includes.

You will end up with a procedural macro `comp!` which mimics python
comprehension lists. That means you can compile code like this:

```rust
let v = vec![1, 2, 3];
for i in comp![ x*x for x in v ] {
    println!("{i}")
}
```

and also with `if` clauses like that

```rust
let v = 1..=10;
for i in comp![ x*x for x in v if x >= 3 if x <= 7 ] {
    println!("{i}")
}
```

Also the following should not compile:

```rust
let v = 1..=10;
// --------------------------------------** not valid in our syntax.
for i in comp![ x*x for x in v if x >= 3 && if x <= 7 ] {
    println!("{i}")
}
```

Happy solving =)
