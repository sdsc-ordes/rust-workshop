#!/usr/bin/env bash
# run inside nix-shell -p nodePackages.node2nix

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
cd "$DIR" || exit 1

cmd="node2nix -10 -i packages.json -c plugins.nix --include-peer-dependencies"

if ! command -v "node2nix" &>/dev/null; then
    nix-shell -p nodePackages.node2nix --run "$cmd"
else
    $cmd
fi
