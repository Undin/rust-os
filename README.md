Rust OS kernel based on https://os.phil-opp.com/ tutorial

## Build, Run and Test

### Requirements
- [rustup](https://rustup.rs/)
- [qemu](https://www.qemu.org/)

### Run
Run `cargo run` or launch `run` run configuration from IDE

### Test
Run `CARGO_PROFILE_DEV_PANIC=unwind cargo test` or launch `test` run configuration from IDE.
Note, running tests from IDE requires disabling Test View (`org.rust.cargo.test.tool.window` [experimental feature](https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-faq.html#experimental-features)) 
