set shell := ["bash", "-cue"]
set positional-arguments
set dotenv-load := true
root_dir := `git rev-parse --show-toplevel`
cargo_watch := root_dir + / ".cargo/bin/cargo-watch"

CONTAINER_MGR := env("CONTAINER_MGR", "docker")

# Default recipe to list all recipes.
default:
  just --list

# Enter a DevContainer shell.
develop *args:
  #!/usr/bin/env bash
  echo "Starting developer shell in '.devcontainer'."
  container_mgr="{{CONTAINER_MGR}}"
  devcontainer up --docker-path "$container_mgr" --workspace-folder . && \
  devcontainer exec --docker-path "$container_mgr" --workspace-folder . bash

# Enter a Nix development shell.
nix-develop *args:
    echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"

# List all exercises.
list-exercises:
  @cd "{{root_dir}}/exercises" && \
    echo "Exercises:" && \
    for i in $(find . -mindepth 1 -maxdepth 1); do \
      echo "- '$(basename "$i")'"; \
    done

# Build the exercise with name `name`.
build name *args:
  dir="{{root_dir}}/exercises/{{name}}" && \
    just check-exercise-dir "$dir" && \
    cd "$dir" && cargo build "${@:2}"

test name *args:
  dir="{{root_dir}}/exercises/{{name}}" && \
    just check-exercise-dir "$dir" && \
    cd "$dir" && cargo test "${@:2}"

run name *args:
  dir="{{root_dir}}/exercises/{{name}}" && \
    just check-exercise-dir "$dir" && \
    cd "$dir" && cargo run "${@:2}"

# Continuously watch and build/check/run/test the exercise
# with name `name`.
# Usage: `just watch build basic-syntax --bin 01`
watch type name *args: assert-cargo-watch
  dir="{{root_dir}}/exercises/{{name}}" && \
    just check-exercise-dir "$dir" && \
    cd "$dir" && "{{cargo_watch}}" -- cargo "{{type}}" "${@:3}"

# Shows the solution on the branch `feat/solutions`
# for the specific file `path`
show-solution path:
    git diff HEAD...feat/solutions -- "{{path}}"

[private]
assert-cargo-watch:
  #!/usr/bin/env bash
  if [ -f "{{cargo_watch}}" ]; then
    echo "Cargo watch exists at: '{{cargo_watch}}'."
  fi

  rm "{{cargo_watch}}"

  if command -v cargo-watch &>/dev/null; then
    echo "Cargo watch already installed. Creating symlink."
    mkdir -p "{{root_dir}}/.cargo/bin" &&
      ln -s "$(which cargo-watch)" "{{root_dir}}/.cargo/bin"
  elif [ -f "{{cargo_watch}}" ]; then
    echo "Install 'cargo-watch' in '{{cargo_watch}}'."
    cargo install -q --root "{{root_dir}}/.cargo" cargo-watch ||
      echo "Could not install cargo-watch." >&2;
  fi


[private]
check-exercise-dir dir:
  #!/usr/bin/env bash
  dir="{{dir}}"
  [ -d "$dir" ] || {
    echo "Exercise '$dir' does not exist!"
    echo "Choose one of the following:"
    just list-exercises
    exit 1
  }


[private]
update-vendir:
  #!/usr/bin/env bash
  vendir -f tools/configs/vendir/vendir.yaml \
         --lock-file tools/configs/vendir/vendir.lock.yaml sync
