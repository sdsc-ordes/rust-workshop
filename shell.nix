(builtins.getFlake ("git+file://" + toString ./. + "?dir=tools/nix")).devShells.${builtins.currentSystem}.default
