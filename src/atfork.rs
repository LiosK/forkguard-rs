//! A fork detector implementation using `pthread_atfork()`.

use std::{error, fmt, num, sync, sync::atomic};

static FORK_COUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

/// A fork guard that detects whether the current process has been forked using `pthread_atfork()`.
///
/// This implementation reads a global counter that is incremented by a fork handler registered via
/// `pthread_atfork()`. Each `Guard` instance tracks the last observed value of this counter.
#[derive(Debug)]
pub struct Guard {
    last_fork_count: usize,
}

impl Default for Guard {
    /// Creates a new `Guard` initialized to the current fork count.
    ///
    /// The first call to this method (or [`Guard::try_new`]) will register the global
    /// `pthread_atfork()` handler.
    ///
    /// # Panics
    ///
    /// Panics if `pthread_atfork` fails. Use [`Guard::try_new()`] for a non-panicking version.
    fn default() -> Self {
        Self::try_new().unwrap()
    }
}

impl Guard {
    /// Creates a new `Guard` initialized to the current fork count.
    ///
    /// This function registers a global `pthread_atfork()` handler on the first call.  The handler
    /// increments a global counter in the child process immediately after a fork occurs.
    ///
    /// # Errors
    ///
    /// Returns an [`AtforkError`] if the call to `pthread_atfork()` fails.
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

    /// Returns `true` if a fork has been detected since the last check.
    ///
    /// This method compares the internal state of the guard with the global fork counter. If they
    /// differ, it updates the internal state and returns `true`. Subsequent calls will return
    /// `false` until another fork occurs.
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
