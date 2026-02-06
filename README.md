## openinfer-dsl

Rust-embedded DSL for defining OpenInfer graphs with explicit control flow and
memory semantics. The DSL produces a graph representation that is validated in
the simulator and later lowered by the synthesizer.

### Build
```bash
cargo check
```

### Tests
```bash
cargo test
```

### Notes
- This crate is a proc-macro and is consumed by `openinfer-simulator`.
- Doctests are disabled (the examples depend on the simulator crate).

Docs: https://github.com/arsalan-awnari/openinfer/tree/main/docs/sphinx/modules/openinfer-dsl
