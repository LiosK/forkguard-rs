//! A lightweight crate for detecting process forks.
//!
//! This crate provides a `Guard` that can detect if the current process has been
//! forked since the last check. This is useful for resetting state (like random
//! number generators or connection pools) that should not be shared between a
//! parent and its forked child.
//!
//! # Examples
//!
//! ```rust
//! let mut guard = forkguard::new();
//!
//! // Some time later...
//! if guard.detected_fork() {
//!     // Handle the fork (e.g., re-initialize state)
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(unix, feature = "atfork"))]
pub mod atfork;
pub mod noop;
pub mod pid;

#[cfg(not(unix))]
pub use noop::Guard;

#[cfg(all(unix, not(feature = "atfork")))]
pub use pid::Guard;

#[cfg(all(unix, feature = "atfork"))]
pub use atfork::Guard;

/// Creates a new fork `Guard` instance.
///
/// The behavior of the returned `Guard` depends on the platform and enabled features:
///
/// - **Unix with `atfork` feature:** Returns [`atfork::Guard`], which uses `pthread_atfork()` to
///   detect forks.
/// - **Unix without `atfork` feature:** Returns [`pid::Guard`], which tracks changes in the
///   process ID.
/// - **Non-Unix platforms:** Returns [`noop::Guard`], which never detects a fork.
pub fn new() -> Guard {
    Guard::default()
}
