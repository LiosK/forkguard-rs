# forkguard

[![Crates.io](https://img.shields.io/crates/v/forkguard)](https://crates.io/crates/forkguard)
[![License](https://img.shields.io/crates/l/forkguard)](https://github.com/LiosK/forkguard-rs/blob/main/LICENSE)

A lightweight crate for detecting process forks.

This crate provides a `Guard` that can detect if the current process has been
forked since the last check. This is useful for resetting state (like random
number generators or connection pools) that should not be shared between a
parent and its forked child.

## Examples

```rust
let mut guard = forkguard::new();

// Some time later...
if guard.detected_fork() {
    // Handle the fork (e.g., re-initialize state)
}
```

## License

Licensed under the Apache License, Version 2.0.
