set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`

# Enter a Nix development shell.
nix-develop *args:
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --command "${cmd[@]}"

generate:
  #!/usr/bin/env bash
  set -eu

  root_dir="{{root_dir}}"
  rm -rf "build"
  mkdir -p "build"

  build_dir="$root_dir/build"
  cd "{{root_dir}}/external/teach-rs/modmod" &&
    cargo run -- generate -o "$build_dir" -c "$root_dir/src/content/rust-for-python.toml"

  cd "$build_dir" && npm install
