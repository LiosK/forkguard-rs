//! ```rust
//! let mut guard = forkguard::new();
//!
//! if guard.detected_fork() {
//!     // ...
//! }
//! ```

pub mod noop;
pub mod pid;

#[cfg(feature = "atfork")]
pub mod atfork;

#[cfg(not(unix))]
pub use noop::Guard;

#[cfg(all(unix, not(feature = "atfork")))]
pub use pid::Guard;

#[cfg(all(unix, feature = "atfork"))]
pub use atfork::Guard;

/// Returns a new fork `Guard` instance.
///
/// On Unix:
/// - If the `atfork` feature is enabled, returns [`atfork::Guard`] (using `pthread_atfork()`).
/// - Otherwise, returns [`pid::Guard`] (tracking process ID changes).
///
/// On non-Unix platforms:
/// - Returns [`noop::Guard`] (never detects a fork).
pub fn new() -> Guard {
    Guard::default()
}
