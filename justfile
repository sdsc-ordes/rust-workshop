set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`
build_dir := root_dir / "build"
source_dir := root_dir / "src"

# Enter a Nix development shell.
nix-develop *args:
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --command "${cmd[@]}"


init:
  #!/usr/bin/env bash
  just generate


generate *args:
  just generate-slides --clear "$@"
  just init-node-modules

# Watch the files in `src` and synchronize them into the `build` folder.
watch:
    #!/usr/bin/env bash
    set -eu

    root_dir="{{root_dir}}"
    build_dir="{{build_dir}}"

    cd "$root_dir"

    if [ ! -d "$build_dir/slides/node_modules" ] ||
       [ ! -f "$build_dir/slides/yarn.lock" ]; then
      echo "Run first 'just generate'."
      exit 1
    fi

    watch() {
      echo "Starting watchman ..."
      watchman-wait -m 0 -t 0 "$@"
    }

    checksum_dir=build/.checksums
    mkdir -p "$checksum_dir"

    watch src tools | (
      while true; do
        read -t 1 LINE && { echo "Watchman: $LINE"; } || continue

        if [ ! -f "$LINE" ]; then
          continue
        fi

        # Ignore some stupid files.
        if echo "$LINE" | grep ".temp.pandoc-include"; then
          continue
        fi

        key=$(echo "$LINE" | sha1sum | cut -f 1 -d ' ')
        current_hash=$(sha1sum "$LINE" | cut -f 1 -d ' ')

        if [ -f "$checksum_dir/$key" ]; then
          if [ "$(cat "$checksum_dir/$key")" = "$current_hash" ]; then
            echo "No changes detected."
            continue
          fi
        fi

        # Store file hash.
        echo "$current_hash" > "$checksum_dir/$key"

        echo "File: '$LINE' changes"
        just patch
      done
    )

patch *args:
  #!/usr/bin/env bash
  set -eu
  root_dir="{{root_dir}}"
  build_dir="{{build_dir}}"

  just generate-slides \
      --patch "$build_dir/.patch-file"

  cd "$build_dir" && git apply .patch-file

build:
  #!/usr/bin/env bash
  set -eu
  root_dir="{{root_dir}}"
  build_dir="{{build_dir}}"

  # Build all slides
  cd "$build_dir/slides"
  for target in $(grep -o 'build-[^"]*' package.json); do
    npm run "$target"
  done

serve module="1_1":
  #!/usr/bin/env bash
  set -eu
  root_dir="{{root_dir}}"
  build_dir="{{build_dir}}"

  cd "$build_dir/slides"
  npm run "dev-{{module}}"

[private]
generate-slides *args:
  #!/usr/bin/env bash
  set -eu
  root_dir="{{root_dir}}"
  build_dir="{{build_dir}}"
  source_dir="{{source_dir}}"

  cd "$root_dir/external/teach-rs/modmod" &&
    cargo run -- generate \
      -o "$build_dir" \
      --theme seriph \
      --json-stub "$source_dir/content/package.json" \
      "$@" \
      "$source_dir/content/rust-for-python.toml"

edit-theme:
  #!/usr/bin/env bash
  set -eu
  build_dir="{{build_dir}}"
  cd "$build_dir/slides" && npm run

[private]
init-node-modules:
  cd "{{build_dir}}/slides" && yarn install
