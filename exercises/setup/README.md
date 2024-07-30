# Trying it out

Now that you've got the code on your machine, navigate to it using your favorite
terminal and run:

```
cd #[modmod:exercise_dir]
cargo run
```

This command may take a while to run the first time, as Cargo will first fetch
the crate index from the registry. It will compile and run the `intro` package,
which you can find in `#[modmod:exercise_dir]`. If everything goes well, you
should see some output:

```
   Compiling intro v0.1.0 ([REDACTED]/#[modmod:exercise_dir])
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/intro`
🦀 Hello, world! 🦀
You've successfully compiled and run your first Rust project!
```

If Rust-Analyzer is set up correctly, you can also click the '▶️ Run'-button
that is shown in `#[modmod:exercise_dir]/src/main.rs`. With CodeLLDB installed
correctly, you can also start a debug session by clicking 'Debug', right next to
the '▶️ Run'-button. Play a little with setting breakpoints by clicking on a
line number, making a red circle appear and stepping over/into/out of functions
using the controls. You can view variable values by hovering over them while
execution is paused, or by expanding the 'Local' view under 'Variables' in the
left panel during a debug session.
