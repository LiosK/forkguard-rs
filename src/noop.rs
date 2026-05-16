//! A no-op fork detector implementation.

use std::{convert, fmt};

use crate::Flavor;

/// A no-op fork guard.
///
/// This implementation does nothing and always returns `false` from [`detected_fork()`]. It is
/// typically used on platforms where fork detection is either not supported or not required.
///
/// [`detected_fork()`]: Guard::detected_fork
#[derive(Default, Clone)]
pub struct Guard {
    _private: (),
}

impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Guard")
            .field("flavor()", &self.flavor())
            .finish()
    }
}

impl Guard {
    /// The flavor of this guard type.
    pub const FLAVOR: Flavor = Flavor::Noop;

    /// Creates a new `Guard` instance.
    pub fn try_new() -> Result<Self, convert::Infallible> {
        Ok(Default::default())
    }

    /// Always returns `false`.
    #[inline(always)]
    pub fn detected_fork(&mut self) -> bool {
        false
    }

    /// Returns the flavor of this guard instance.
    pub fn flavor(&self) -> Flavor {
        Self::FLAVOR.clone()
    }
}
