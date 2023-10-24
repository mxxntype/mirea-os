use std::cell::RefCell;

use crate::page::{PAGE_COUNT, PAGE_SIZE};

pub const RAM_SIZE: usize = PAGE_SIZE * PAGE_COUNT;

/// RAM model. Owns the bytes.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ram {
    pub bytes: RefCell<[u8; RAM_SIZE]>,
}

impl Ram {
    /// Creates a new RAM object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: RefCell::new([0; RAM_SIZE]),
        }
    }
}
