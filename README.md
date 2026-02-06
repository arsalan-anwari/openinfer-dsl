## openinfer-dsl

Rust-embedded DSL for defining OpenInfer graphs with explicit control flow and
memory semantics. The DSL produces a graph representation that is validated in
the simulator and later lowered by the synthesizer.

### Build
```bash
cargo check
cargo build -p openinfer-dsl
```

### Notes
- This crate is a proc-macro and is consumed by `openinfer-simulator`.
- Doctests are disabled (the examples depend on the simulator crate).

Docs: docs.open-infer.nl
