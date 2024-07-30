#!/usr/bin/env bash
set -e
set -u

# shellcheck disable=2016
echo 'eval "$(direnv hook bash)"' >>~/.bashrc
nix develop ./tools/nix#default --accept-flake-config --command echo \"Done caching Nix dev. shell.\"
