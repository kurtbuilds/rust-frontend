use std::ops::{Deref, DerefMut};

/// This isn't actually used, but is a generic example of how to implement the field observable pattern
/// This is used by the DocumentObserver struct.

struct ObserverLock<T, F> {
    inner: T,
    callback: F,
}

impl<T, F> ObserverLock<T, F>
    where
        F: FnMut(&mut T),
{
    pub fn new(inner: T, callback: F) -> Self {
        ObserverLock { inner, callback }
    }

    pub fn lock(&mut self) -> ObserverLockGuard<T, F> {
        ObserverLockGuard { lock: self }
    }
}

struct ObserverLockGuard<'a, T, F: FnMut(&mut T)> {
    lock: &'a mut ObserverLock<T, F>,
}

impl<'a, T, F: FnMut(&mut T)> Deref for ObserverLockGuard<'a, T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<'a, T, F: FnMut(&mut T)> DerefMut for ObserverLockGuard<'a, T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.inner
    }
}

impl<'a, T, F: FnMut(&mut T)> Drop for ObserverLockGuard<'a, T, F> {
    fn drop(&mut self) {
        (self.lock.callback)(&mut self.lock.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut lock = ObserverLock::new(0, |state| println!("Changed to: {}", state));

        {
            let mut guard = lock.lock();
            *guard += 1;
        }

        // These two modifications will be collapsed into one
        {
            let mut guard = lock.lock();
            *guard += 2;
            *guard += 3;
        }
    }
}