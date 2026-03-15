//! ```rust
//! let mut guard = forkguard::new();
//!
//! if guard.detected_fork() {
//!     // ...
//! }
//! ```

pub mod noop;
pub mod pid;

#[cfg(unix)]
pub use pid::Guard;

#[cfg(not(unix))]
pub use noop::Guard;

/// Returns an instance of [`pid::Guard`] on Unix or [`noop::Guard`] otherwise.
pub fn new() -> Guard {
    Guard::default()
}
