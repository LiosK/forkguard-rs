//! A fork detector implementation using `pthread_atfork()`.

use std::{error, fmt, num, sync, sync::atomic};

static FORK_COUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

/// A fork guard that detects process forks using `pthread_atfork()`.
///
/// This implementation registers a fork handler that increments a global counter when a fork
/// occurs. The guard detects a fork by checking if this counter has changed since the last call to
/// [`detected_fork()`]. The fork handler is registered via `pthread_atfork()` only for the child
/// process, so `detected_fork()` will return `true` only in the forked child, matching the
/// behavior of [`pid::Guard`].
///
/// [`detected_fork()`]: Guard::detected_fork
/// [`pid::Guard`]: crate::pid::Guard
#[derive(Debug)]
pub struct Guard {
    last_fork_count: usize,
}

impl Default for Guard {
    /// Creates a new `Guard` instance.
    ///
    /// The first call to this function (or [`Guard::try_new()`]) registers the fork handler via
    /// `pthread_atfork()`.
    ///
    /// # Panics
    ///
    /// Panics if `pthread_atfork()` fails, which is extremely unlikely under normal conditions.
    fn default() -> Self {
        Self::try_new().unwrap()
    }
}

impl Guard {
    /// Creates a new `Guard` instance.
    ///
    /// The first call to this function (or [`Guard::default()`]) registers the fork handler via
    /// `pthread_atfork()`.
    ///
    /// # Errors
    ///
    /// Returns an error if `pthread_atfork()` fails, which is extremely unlikely under normal
    /// conditions.
    pub fn try_new() -> Result<Self, AtforkError> {
        static ATFORK_RESULT: sync::LazyLock<libc::c_int> =
            sync::LazyLock::new(|| unsafe { libc::pthread_atfork(None, None, Some(fork_handler)) });
        extern "C" fn fork_handler() {
            FORK_COUNT.fetch_add(1, atomic::Ordering::Relaxed);
        }

        match *ATFORK_RESULT {
            0 => Ok(Self {
                last_fork_count: FORK_COUNT.load(atomic::Ordering::Relaxed),
            }),
            ret => Err(AtforkError(ret.try_into().unwrap())),
        }
    }

    /// Returns `true` in the child process if a fork has occurred since the last call to this
    /// function. Otherwise, returns `false`.
    #[inline(always)]
    pub fn detected_fork(&mut self) -> bool {
        let current_fork_count = FORK_COUNT.load(atomic::Ordering::Relaxed);
        if self.last_fork_count == current_fork_count {
            false
        } else {
            self.set_fork_count(current_fork_count);
            true
        }
    }

    #[cold]
    fn set_fork_count(&mut self, value: usize) {
        self.last_fork_count = value;
    }
}

/// An error returned when `pthread_atfork()` fails.
#[derive(Debug)]
pub struct AtforkError(num::NonZero<libc::c_int>);

impl AtforkError {
    /// Returns the raw error code from `pthread_atfork()`.
    pub fn code(&self) -> num::NonZero<libc::c_int> {
        self.0
    }
}

impl fmt::Display for AtforkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "pthread_atfork() failed (code: {})", self.0)
    }
}

impl error::Error for AtforkError {}
