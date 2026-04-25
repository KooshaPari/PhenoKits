//! Minimal FFI utilities for Python/Rust interop
//! Provides FfiMutex wrapper around parking_lot::Mutex for PyO3 integration

use parking_lot::Mutex;

/// Thread-safe mutex wrapper suitable for FFI boundaries
pub struct FfiMutex<T>(Mutex<T>);

impl<T> FfiMutex<T> {
    /// Create a new FfiMutex protecting the given value
    pub const fn new(val: T) -> Self {
        FfiMutex(Mutex::new(val))
    }

    /// Acquire lock and return guard
    pub fn lock(&self) -> parking_lot::MutexGuard<'_, T> {
        self.0.lock()
    }
}

impl<T: Default> Default for FfiMutex<T> {
    fn default() -> Self {
        FfiMutex::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_mutex_new() {
        let m = FfiMutex::new(42);
        assert_eq!(*m.lock(), 42);
    }

    #[test]
    fn test_ffi_mutex_mutation() {
        let m = FfiMutex::new(vec![1, 2, 3]);
        m.lock().push(4);
        assert_eq!(m.lock().len(), 4);
    }
}
