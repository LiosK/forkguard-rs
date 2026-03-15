//! A fork detector implementation based on tracking the current process ID.

use std::process;

/// A fork guard that detects process ID changes.
///
/// This implementation tracks the current process ID and returns `true` from [`detected_fork()`]
/// if the process ID has changed since the last check.
///
/// [`detected_fork()`]: Guard::detected_fork
#[derive(Debug)]
pub struct Guard {
    last_pid: u32,
}

impl Default for Guard {
    fn default() -> Self {
        let last_pid = process::id();
        Self { last_pid }
    }
}

impl Guard {
    /// Returns `true` if the current process ID has changed since the last call to this function.
    /// Otherwise, returns `false`.
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
}
