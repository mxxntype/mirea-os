use crate::page::{MAX_PAGE_COUNT, PAGE_SIZE};
use std::cell::RefCell;

pub const RAM_SIZE: usize = PAGE_SIZE * MAX_PAGE_COUNT;

/// RAM model. Owns the bytes.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ram {
    pub(crate) bytes: RefCell<[u8; RAM_SIZE]>,
}

impl Default for Ram {
    /// Creates a default [`Ram`] object.
    fn default() -> Self {
        Self {
            bytes: RefCell::new([0; RAM_SIZE]),
        }
    }
}
