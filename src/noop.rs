//! A no-op fork guard implementation.

/// A no-op fork guard.
///
/// This implementation always returns `false` from [`detected_fork()`]. This is typically used on
/// platforms where fork detection is either not supported or not required.
///
/// [`detected_fork()`]: Guard::detected_fork
#[derive(Debug, Default)]
pub struct Guard {
    _private: (),
}

impl Guard {
    /// Returns `false` always.
    #[inline(always)]
    pub fn detected_fork(&mut self) -> bool {
        false
    }
}
