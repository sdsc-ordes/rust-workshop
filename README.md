# Rust Workshop

This repository contains exercises accompanied by the
[Rust Workshop Presentation](https://github.com/sdsc-ordes/technical-presentation/tree/gabriel.nuetzi/rust-workshop).

(**Presentation Slides:
here.**)[https://sdsc-ordes.github.io/technical-presentation/gh-pages/rust-workshop]

(**Excercise Solution Branch:
here.**)[https://github.com/sdsc-ordes/rust-workshop/tree/feat/solutions]

## Preliminaries

- Read the [setup guide](/docs/setup.md) and make sure you can compile the test
  application.

## Build & Test & Run Exercises

Any exercise has a small `README.md` with additional information to it.

To see all exercises use:

```shell
just list-exercises
```

You can build any exercise in [`./exercises`](./exercises) with the tool `cargo`
by doing

```shell
cd ./exercises/basic-syntax
cargo build
```

Use `cargo test`, `cargo run` to test and run the executables in the small
exercise projects.

<!-- prettier-ignore -->
> [!NOTE]
> An exercise might contain multiple executables:
> By default all are built. Use `--bin <executable-name>`
> to build a specific one, e.g. `cargo build --bin 01` in the above example.

<!-- prettier-ignore -->
> [!TIP]
> Use the nicer `just` targets:
>
> ```shell
>    just build <exercise-name> [build|test|check|run] [add-cargo-args...]`
> ```
>
> so you can do
>
> ```shell
> just build basic-syntax --bin 01
> ```
>
> or
>
> ```shell
> just watch [build|test|check|run] basic-syntax --bin 01
> ```
>
> to **continuously build/test/run** any exercise.

## Solutions

The solutions to most exercises are on the branch `feat/solutions` and you can
simply use `git diff HEAD...feat/solution -- <path>` where `<path>` is the path
to a Rust file you are interested in looking at the solution.

## Acknowledgment

Special thanks to [teach-rs](https://github.com/trifectatechfoundation/teach-rs)
for providing exercises and slides which this whole workshop is based on.

Also some parts have been taken from
[comprehensive-rust](https://google.github.io/comprehensive-rust/). This course
is more tailored towards C/C++ intermediates.
