//! A lightweight crate for detecting process forks.
//!
//! ```rust
//! let mut guard = forkguard::new();
//!
//! // Some time later...
//! if guard.detected_fork() {
//!     // Handle the fork (e.g., re-initialize state)
//! }
//! ```
//!
//! This crate provides `Guard` types that can detect if the current process has
//! been forked since the last check. This is useful for resetting state (like
//! random number generators or connection pools) that should not be shared between
//! a parent and its forked child.
//!
//! Depending on the platform and enabled features, [`new()`] returns one of three
//! specialized `Guard` flavors:
//!
//! - [`atfork::Guard`] (Unix with `atfork` crate feature): Uses `pthread_atfork()` to
//!   update the global state on fork. This is the most efficient detection
//!   mechanism for supported Unix-like systems, though the usual caveats of fork
//!   handlers apply.
//! - [`pid::Guard`] (Unix default): Tracks the process ID and detects changes. This
//!   is a simple and sufficiently efficient option for Unix-like platforms unless
//!   `getpid()` overhead is a concern.
//! - [`noop::Guard`] (Non-Unix): Provides a no-op implementation that always returns
//!   `false`, used on platforms where fork detection is not required.
//!
//! # Crate features
//!
//! - `atfork` (optional): See the above description.

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
/// - Unix with `atfork` feature: Returns [`atfork::Guard`], which uses `pthread_atfork()` to
///   detect forks.
/// - Unix without `atfork` feature: Returns [`pid::Guard`], which tracks changes in the process
///   ID.
/// - Non-Unix platforms: Returns [`noop::Guard`], which never detects a fork.
pub fn new() -> Guard {
    Guard::default()
}
