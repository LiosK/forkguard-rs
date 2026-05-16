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
//! The top-level `forkguard::Guard` resolves to the active flavor.
//!
//! # Crate features
//!
//! - `atfork` (optional): See the above description.
//!
//! This crate lets downstream users opt-in to the `atfork` feature even when the
//! intermediate dependencies do not. To do this, add the following to your
//! `Cargo.toml` no matter if your crate uses `forkguard` directly:
//!
//! ```toml
//! [dependencies]
//! forkguard = { version = "0.1", features = ["atfork"] }
//! ```
//!
//! Alternatively, add the following to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! forkguard = "0.1"
//! ```
//!
//! And then enable the feature in your build command:
//!
//! ```bash
//! cargo build --features forkguard/atfork
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
/// The actual type of the returned `Guard` depends on the platform and enabled features:
///
/// - Unix with `atfork` feature: Returns [`atfork::Guard`], which uses `pthread_atfork()` to
///   detect forks.
/// - Unix without `atfork` feature: Returns [`pid::Guard`], which tracks changes in the process
///   ID.
/// - Non-Unix platforms: Returns [`noop::Guard`], which never detects a fork.
///
/// # Panics
///
/// Panics if `atfork::Guard` cannot be created due to `pthread_atfork()` failure, which is
/// extremely unlikely under normal conditions. `pid::Guard` or `noop::Guard` never panics.
///
/// # Examples
///
/// ```rust
/// let mut guard = forkguard::new();
///
/// if guard.detected_fork() {
///     // Handle the fork (e.g., re-initialize state)
/// }
/// ```
#[track_caller]
pub fn new() -> Guard {
    Guard::default()
}

/// A non-panicking variant of [`new()`].
///
/// # Errors
///
/// Returns an error if [`atfork::Guard`] cannot be created due to `pthread_atfork()` failure, which
/// is extremely unlikely under normal conditions. [`pid::Guard`] or [`noop::Guard`] never panics.
///
/// # Examples
///
/// ```rust
/// let mut guard = forkguard::try_new().expect("failed to create fork guard");
///
/// if guard.detected_fork() {
///     // Handle the fork (e.g., re-initialize state)
/// }
/// ```
pub fn try_new() -> Result<Guard, impl std::error::Error> {
    Guard::try_new()
}

/// An enumeration of the different fork detection flavors provided by this crate.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Flavor {
    /// Indicates [`atfork::Guard`].
    Atfork,

    /// Indicates [`pid::Guard`].
    Pid,

    /// Indicates [`noop::Guard`].
    Noop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_has_common_interface() {
        let _ = Guard::FLAVOR;
        let _ = Guard::try_new();
        let mut guard = Guard::default();
        let _ = guard.detected_fork();
        let _ = guard.flavor();
    }

    #[test]
    fn debug_output_contains_flavor() {
        let guard = Guard::default();
        let debug_str = format!("{:?}", guard);
        let flavor_str = format!("{:?}", guard.flavor());
        assert!(debug_str.contains("Guard"));
        assert!(debug_str.contains("flavor()"));
        assert!(debug_str.contains(&flavor_str));
    }

    #[test]
    fn right_flavor_is_active() {
        #[cfg(not(unix))]
        assert_eq!(Guard::FLAVOR, Flavor::Noop);

        #[cfg(all(unix, not(feature = "atfork")))]
        assert_eq!(Guard::FLAVOR, Flavor::Pid);

        #[cfg(all(unix, feature = "atfork"))]
        assert_eq!(Guard::FLAVOR, Flavor::Atfork);

        assert_eq!(new().flavor(), Guard::FLAVOR);
        assert_eq!(try_new().unwrap().flavor(), Guard::FLAVOR);
    }
}
