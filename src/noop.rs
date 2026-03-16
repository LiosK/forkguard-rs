//! A no-op fork detector implementation.

/// A no-op fork guard.
///
/// This implementation does nothing and always returns `false` from [`detected_fork()`]. It is
/// typically used on platforms where fork detection is either not supported or not required.
///
/// [`detected_fork()`]: Guard::detected_fork
#[derive(Debug, Default)]
pub struct Guard {
    _private: (),
}

impl Guard {
    /// Always returns `false`.
    #[inline(always)]
    pub fn detected_fork(&mut self) -> bool {
        false
    }
}
