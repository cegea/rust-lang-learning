# Rust self-learning

This repo contains exercises for learning Rust, extracted from [Google's Comprehensive Rust](https://google.github.io/comprehensive-rust/)

## Cargo commands cheat sheet

```bash
cargo new --vcs=none my-new-project
```

Create new projects without creating a new repo

```bash
cargo build
```

Build a Rust project

```bash
cargo build -j JOBS
```

Build a Rust project in parallel with mutiple jobs

```bash
cargo run
```

Executes benchmark of project, requires tests

```bash
cargo bench
```

Executes benchmark of project, requires tests

```bash
cargo check
```

Analyze the current project and report possible errors, but does not build object files

```bash
cargo test
```

Executes project tests, requires tests

```bash
cargo doc
```

Builds projects documentation, requires inlines documentation in source files

```bash
cargo clean
```

Clean out the build, by removing the target/ directory
