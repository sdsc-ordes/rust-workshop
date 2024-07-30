# Rust Workshop

This repository contains exercises accompanied by the
[Rust Workshop Presentation](https://github.com/sdsc-ordes/technical-presentation/tree/gabriel.nuetzi/rust-workshop).

The presentation slides can be watched here: <!-- TODO: Add here the link -->

## Build & Test & Run Exercises

Any exercise has a small `README.md` with additional information to it.

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
>    just build <exercise-name> [build|test|run|watch] [add-cargo-args...]`
> ```
>
> to build/test/run any exercise. Use **`watch`** to continuously build
> an exercise.

## Preparations Before Workshop

In this section you'll find instructions on how to install the tools we'll use
during the course.

All of these tools are available for Linux, macOS and Windows users. We'll need
the tools to write and compile our Rust code, and allow for remote mentoring.

_Important: These instructions are to be followed before the workshop, before
the start of the first tutorial._ _If you have any problems with installation,
contact the lecturers! We won't be addressing installation problems during the
first tutorial._

There are 3 setups you can choose from which are described further below:

- **Nix Development Shell**: A stable development environment with the Nix
  package manager. The `nix develop ./tools/nix#default` will enter a
  development shell with the toolchain in your `PATH`. The editor setup is on
  your shoulder.

- **[DevContainer](.devcontainer)**: Using a development container with `podman`
  or `docker` (best in VS Code) which includes all bells and whistles. Nothing
  has to be setup. Just launch `code` and enter the DevContainer.

- **Manual Setup**: Install the tools yourself on the system of your choice (not
  recommended).

### Nix Development Shell

#### Installing Nix

If you have not installed `nix` on your Linux or macOS use
[`nix-installer`](https://github.com/DeterminateSystems/nix-installer):

```shell
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

Follow their instruction for more information.

#### Entering the Development Shell

If you have the package manager
[`nix`](https://github.com/DeterminateSystems/nix-installer) installed you can
enter a development setup easily with

```shell
nix ./tools/nix#default
```

or `just nix-develop` or automatically when [`direnv`](https://direnv.net) is
installed and [setup for your shell](https://direnv.net/docs/hook.html) and
`direnv allow` was executed inside the repository.

**Note:** Make sure to enable `flakes` and `nix-command` in
[your `nix` config](https://nixos.wiki/wiki/Flakes#Other_Distros,_without_Home-Manager)

### Dev Container Setup

Open `code` and use `Reopen in Container` to enter the
[`.devcontainer`](.devcontainer/devcontainer.json) workspace.

### Manual Setup

#### Rust and Cargo

First we'll need `rustc`, the standard Rust compiler. `rustc` is generally not
invoked directly, but through `cargo`, the Rust package manager. `rustup` takes
care of installing `rustc` and `cargo`.

This part is easy: go to <https://rustup.rs> and follow the instructions. Please
make sure you're installing the latest default toolchain. Once done, run

```bash
rustc -V && cargo -V
```

The output should be something like this:

```bash
rustc 1.79.0 (129f3b996 2024-06-10)
cargo 1.79.0 (ffa9cf99a 2024-06-03)
```

Using Rustup, you can install Rust toolchains and components. More info:

- <https://rust-lang.github.io/rustup>
- <https://doc.rust-lang.org/cargo>

#### Rustfmt and Clippy

To avoid discussions, Rust provides its own formatting tool, `rustfmt`. We'll
also be using `clippy`, a collection of lints to analyze your code, that catches
common mistakes for you. You'll notice that Rusts `clippy` can be a very helpful
companion. Both `rustfmt` and `clippy` are installed by `rustup` by default.

To run `rustfmt` on your project, execute:

```bash
cargo fmt
```

To run `clippy`:

```bash
cargo clippy
```

More info:

- Rustfmt: <https://github.com/rust-lang/rustfmt>
- Clippy: <https://github.com/rust-lang/rust-clippy>

#### Command Runner

You should install [`just`](https://github.com/casey/just) (a Makefile
alternative which is more sane) on your system to make working with these
exercises more convenient.

#### Language Server `rust-analyzer`

To write and learn effectively, you need the `rust-analyzer` installed. The
installer `rustup` will provide this too. The setup of `rust-analyzer` depends
on your editor. With Visual Studio Code (`code`) it will be setup when following
the section below.

### Visual Studio Code

During the course, you are best using Visual Studio Code (`code`) to write code.
We use `code` to allow for remote collaboration and mentoring during tutorial
sessions.

You can find the installation instructions here:
<https://code.visualstudio.com/>.

We will install some plugins as well. The first one is Rust-Analyzer.
Installation instructions can be found here
<https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>.
Rust-Analyzer provides a lot of help during development and in indispensable
when getting started with Rust.

Another plugin we'll install is Live Share. We will use the plugin to share
screens and provide help during remote tutorial sessions. The extension pack
also contains the Live Share Audio plugin, which allows for audio communication
during share sessions. Installation instructions can be found here:
<https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare>

The last plugin we'll use is CodeLLDB. This plugin enables debugging Rust code
from within vscode. You can find instructions here:
<https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb>.

More info:

- <https://rust-analyzer.github.io/>
- <https://code.visualstudio.com/learn/collaboration/live-share>

### Testing the Setup

Run

```
just run setup
```

```
🦀 Hello, world! 🦀
You've successfully compiled and run your first Rust project!
X: 2; Y: 2
```

If that works meaning it builds and runs the setup binary, you are all set for
the exercises.

## Acknowledgment

Special thanks to [teach-rs](https://github.com/trifectatechfoundation/teach-rs) for providing 
exercises and slides which this whole workshop is based on.

Also some parts have been taken from [comprehensive-rust](https://google.github.io/comprehensive-rust/).
This course is more tailored towards C/C++ intermediates.
