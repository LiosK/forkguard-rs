//! A fork detector implementation based on the process ID.

use std::{convert, fmt, process};

/// A fork guard that detects process forks by tracking process ID changes.
///
/// This implementation detects a fork by checking if the current process ID has changed since the
/// last call to [`detected_fork()`]. Since only the child process receives a new process ID after
/// a fork, `detected_fork()` will return `true` only in the child.
///
/// [`detected_fork()`]: Guard::detected_fork
pub struct Guard {
    last_pid: u32,
}

impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Guard")
            .field("flavor()", &self.flavor())
            .field("last_pid", &self.last_pid)
            .finish()
    }
}

impl Default for Guard {
    fn default() -> Self {
        let last_pid = process::id();
        Self { last_pid }
    }
}

impl Guard {
    /// Creates a new `Guard` instance.
    pub fn try_new() -> Result<Self, convert::Infallible> {
        Ok(Default::default())
    }

    /// Returns `true` in the child process if a fork has occurred since the last call to this
    /// function. Otherwise, returns `false`.
    #[inline(always)]
    pub fn detected_fork(&mut self) -> bool {
        let current_pid = process::id();
        if self.last_pid == current_pid {
            false
        } else {
            self.set_pid(current_pid);
            true
        }
    }

    #[cold]
    fn set_pid(&mut self, value: u32) {
        self.last_pid = value;
    }

    /// Returns the flavor of this guard.
    ///
    /// This method is intended for diagnostic purposes only.
    fn flavor(&self) -> impl fmt::Debug {
        crate::Flavor::Pid
    }
}
