<!-- TODO this exercise imports dependencies, a concept which is introduced later.
    We may want to put that a bit into the background by providing proxy functions in the scaffolding
    which can be called in the implementations of the `DeserializeConfig` trait
-->

In this exercise, you'll work with dynamic dispatch to deserialize with `serde_json` or `serde_yaml`, depending on the file extension. The starter code is in `src/main.rs`. Fix the todo's in there.

To run the program, you'll need to pass the file to deserialize to the binary, not to Cargo. To do this, run
```bash
cargo run -- <path>
```

Deserializing both `config.json` and `config.yml` should result in the `Config` being printed correctly.
