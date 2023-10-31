use crate::page::{MAX_PAGE_COUNT, PAGE_SIZE};
use std::cell::RefCell;

pub const RAM_SIZE: usize = PAGE_SIZE * MAX_PAGE_COUNT;

/// RAM model. Owns the bytes.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ram {
    pub(crate) bytes: RefCell<[u8; RAM_SIZE]>,
}

impl Ram {
    /// Creates a new RAM object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: RefCell::new([0; RAM_SIZE]),
        }
    }

    /// Returns the current usage of [`Ram`] as an `f64`.
    pub fn usage(&self) -> f64 {
        let bytes = self.bytes.borrow();
        let used_bytes = bytes.iter().filter(|x| **x != 0).count();
        used_bytes as f64 / RAM_SIZE as f64
    }
}
